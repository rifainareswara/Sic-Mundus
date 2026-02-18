use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use serde::Serialize;
use crate::{
    db::DbPool,
    models::user::{AuthClaims, ResetPasswordRequest, UpdateProfileRequest, UpdateRoleRequest, User},
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::query_as;

// Helper: extract role from request
fn caller_role(req: &HttpRequest) -> Option<String> {
    req.extensions().get::<AuthClaims>().map(|c| c.role.clone())
}
fn caller_id(req: &HttpRequest) -> Option<String> {
    req.extensions().get::<AuthClaims>().map(|c| c.sub.clone())
}

// Admin/Superadmin: Get users (superadmin sees all, admin sees only users)
pub async fn get_users(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let role = match caller_role(&req) {
        Some(r) => r,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let users = if role == "superadmin" {
        query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
            .fetch_all(pool.get_ref())
            .await
    } else if role == "admin" {
        query_as::<_, User>("SELECT * FROM users WHERE role = 'user' ORDER BY created_at DESC")
            .fetch_all(pool.get_ref())
            .await
    } else {
        return HttpResponse::Forbidden().finish();
    };

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Admin/Superadmin: Delete user (admin can only delete 'user' role)
pub async fn delete_user(pool: web::Data<DbPool>, req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let caller_role = match caller_role(&req) {
        Some(r) => r,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let user_id = path.into_inner();

    // Fetch target user's role
    let target_role: Option<String> = sqlx::query_scalar("SELECT role FROM users WHERE id = $1")
        .bind(&user_id)
        .fetch_optional(pool.get_ref())
        .await
        .unwrap_or(None);

    match target_role.as_deref() {
        None => return HttpResponse::NotFound().finish(),
        Some("superadmin") => return HttpResponse::Forbidden().body("Cannot delete superadmin"),
        Some("admin") if caller_role != "superadmin" => {
            return HttpResponse::Forbidden().body("Only superadmin can delete admin")
        }
        _ => {}
    }

    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Admin/Superadmin: Update user role
pub async fn update_role(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateRoleRequest>,
) -> impl Responder {
    let caller_role = match caller_role(&req) {
        Some(r) => r,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let caller_id = caller_id(&req).unwrap_or_default();
    let user_id = path.into_inner();

    if user_id == caller_id {
        return HttpResponse::BadRequest().body("Cannot change your own role");
    }

    // Validate allowed roles
    let allowed_roles = if caller_role == "superadmin" {
        vec!["superadmin", "admin", "user"]
    } else if caller_role == "admin" {
        vec!["user"] // admin can only toggle between user roles
    } else {
        return HttpResponse::Forbidden().finish();
    };

    if !allowed_roles.contains(&body.role.as_str()) {
        return HttpResponse::BadRequest().body("Invalid role or insufficient permissions");
    }

    // Fetch target user's current role
    let target_role: Option<String> = sqlx::query_scalar("SELECT role FROM users WHERE id = $1")
        .bind(&user_id)
        .fetch_optional(pool.get_ref())
        .await
        .unwrap_or(None);

    match target_role.as_deref() {
        None => return HttpResponse::NotFound().finish(),
        Some("superadmin") => return HttpResponse::Forbidden().body("Cannot change superadmin role"),
        Some("admin") if caller_role != "superadmin" => {
            return HttpResponse::Forbidden().body("Only superadmin can change admin role")
        }
        _ => {}
    }

    let result = sqlx::query("UPDATE users SET role = $1 WHERE id = $2")
        .bind(&body.role)
        .bind(user_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Admin only: Reset password
pub async fn reset_password(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    body: web::Json<ResetPasswordRequest>,
) -> impl Responder {
    let user_id = path.into_inner();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(body.new_password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    let result = sqlx::query("UPDATE users SET password_hash = $1, force_change_password = TRUE WHERE id = $2")
        .bind(password_hash)
        .bind(user_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Current user: Update profile (full_name)
pub async fn update_profile(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<UpdateProfileRequest>,
) -> impl Responder {
    let user_id = {
        let ext = req.extensions();
        match ext.get::<AuthClaims>() {
            Some(c) => c.sub.clone(),
            None => return HttpResponse::Unauthorized().finish(),
        }
    };

    let result = sqlx::query("UPDATE users SET full_name = $1 WHERE id = $2")
        .bind(&body.full_name)
        .bind(&user_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Admin only: Time report — total minutes per user per project
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TimeReportRow {
    pub user_id: String,
    pub username: String,
    pub full_name: String,
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub project_color: Option<String>,
    pub total_minutes: i64,
}

pub async fn get_time_report_admin(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let role = {
        let ext = req.extensions();
        match ext.get::<AuthClaims>() {
            Some(c) => c.role.clone(),
            None => return HttpResponse::Unauthorized().finish(),
        }
    };
    if role != "admin" && role != "superadmin" {
        return HttpResponse::Forbidden().body("Admin only");
    }

    // superadmin sees everyone; admin sees only 'user' role
    let sql = if role == "superadmin" {
        "SELECT u.id AS user_id, u.username, u.full_name,
                p.id AS project_id, p.name AS project_name, p.color AS project_color,
                COALESCE(SUM(e.duration_minutes), 0)::BIGINT AS total_minutes
         FROM users u
         LEFT JOIN time_entries e ON e.user_id = u.id
         LEFT JOIN tasks t ON t.id = e.task_id
         LEFT JOIN projects p ON p.id = t.project_id
         GROUP BY u.id, u.username, u.full_name, p.id, p.name, p.color
         ORDER BY u.username, total_minutes DESC"
    } else {
        "SELECT u.id AS user_id, u.username, u.full_name,
                p.id AS project_id, p.name AS project_name, p.color AS project_color,
                COALESCE(SUM(e.duration_minutes), 0)::BIGINT AS total_minutes
         FROM users u
         LEFT JOIN time_entries e ON e.user_id = u.id
         LEFT JOIN tasks t ON t.id = e.task_id
         LEFT JOIN projects p ON p.id = t.project_id
         WHERE u.role = 'user'
         GROUP BY u.id, u.username, u.full_name, p.id, p.name, p.color
         ORDER BY u.username, total_minutes DESC"
    };

    let result = query_as::<_, TimeReportRow>(sql)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => {
            eprintln!("get_time_report_admin error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
