use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;
use sqlx::{query, query_as};

use crate::db::DbPool;
use crate::models::task::*;

pub async fn get_tasks(pool: web::Data<DbPool>) -> impl Responder {
    let tasks = query_as::<_, Task>(
        "SELECT t.id, t.title, t.description, t.category, t.status, t.due_date, t.start_date, t.created_at, t.updated_at,
                COALESCE(SUM(e.duration_minutes), 0)::BIGINT as total_minutes,
                COUNT(e.id)::BIGINT as entry_count
         FROM tasks t
         LEFT JOIN time_entries e ON e.task_id = t.id
         GROUP BY t.id
         ORDER BY t.updated_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await;

    match tasks {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_task(
    pool: web::Data<DbPool>,
    body: web::Json<CreateTaskRequest>,
) -> impl Responder {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let desc = body.description.clone().unwrap_or_default();
    let cat = body.category.clone().unwrap_or_else(|| "General".into());

    let result = query(
        "INSERT INTO tasks (id, title, description, category, status, due_date, start_date, created_at, updated_at) VALUES ($1, $2, $3, $4, 'pending', $5, $6, $7, $8)"
    )
    .bind(&id)
    .bind(&body.title)
    .bind(&desc)
    .bind(&cat)
    .bind(&body.due_date)
    .bind(&body.start_date)
    .bind(&now)
    .bind(&now)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let task = Task {
                id,
                title: body.title.clone(),
                description: desc,
                category: cat,
                status: "pending".into(),
                due_date: body.due_date.clone(),
                start_date: body.start_date.clone(),
                created_at: now.clone(),
                updated_at: now,
                total_minutes: 0,
                entry_count: 0,
            };
            HttpResponse::Created().json(task)
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_task(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    body: web::Json<UpdateTaskRequest>,
) -> impl Responder {
    let task_id = path.into_inner();
    let now = Utc::now().to_rfc3339();

    if let Some(ref title) = body.title {
        if query("UPDATE tasks SET title = $1, updated_at = $2 WHERE id = $3")
            .bind(title).bind(&now).bind(&task_id)
            .execute(pool.get_ref()).await.is_err() { return HttpResponse::InternalServerError().finish(); }
    }
    if let Some(ref desc) = body.description {
        if query("UPDATE tasks SET description = $1, updated_at = $2 WHERE id = $3")
            .bind(desc).bind(&now).bind(&task_id)
            .execute(pool.get_ref()).await.is_err() { return HttpResponse::InternalServerError().finish(); }
    }
    if let Some(ref cat) = body.category {
        if query("UPDATE tasks SET category = $1, updated_at = $2 WHERE id = $3")
            .bind(cat).bind(&now).bind(&task_id)
            .execute(pool.get_ref()).await.is_err() { return HttpResponse::InternalServerError().finish(); }
    }
    if let Some(ref status) = body.status {
        if query("UPDATE tasks SET status = $1, updated_at = $2 WHERE id = $3")
            .bind(status).bind(&now).bind(&task_id)
            .execute(pool.get_ref()).await.is_err() { return HttpResponse::InternalServerError().finish(); }
    }
    if let Some(ref due_date) = body.due_date {
        let date_val = if due_date.is_empty() { None } else { Some(due_date) };
        if query("UPDATE tasks SET due_date = $1, updated_at = $2 WHERE id = $3")
            .bind(date_val).bind(&now).bind(&task_id)
            .execute(pool.get_ref()).await.is_err() { return HttpResponse::InternalServerError().finish(); }
    }
    if let Some(ref start_date) = body.start_date {
        let date_val = if start_date.is_empty() { None } else { Some(start_date) };
        if query("UPDATE tasks SET start_date = $1, updated_at = $2 WHERE id = $3")
            .bind(date_val).bind(&now).bind(&task_id)
            .execute(pool.get_ref()).await.is_err() { return HttpResponse::InternalServerError().finish(); }
    }

    let task = query_as::<_, Task>(
        "SELECT t.id, t.title, t.description, t.category, t.status, t.due_date, t.start_date, t.created_at, t.updated_at,
                COALESCE((SELECT SUM(duration_minutes) FROM time_entries WHERE task_id = t.id), 0)::BIGINT as total_minutes,
                COALESCE((SELECT COUNT(*) FROM time_entries WHERE task_id = t.id), 0)::BIGINT as entry_count
         FROM tasks t WHERE t.id = $1"
    )
    .bind(&task_id)
    .fetch_one(pool.get_ref())
    .await;

    match task {
        Ok(t) => HttpResponse::Ok().json(t),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_task(pool: web::Data<DbPool>, path: web::Path<String>) -> impl Responder {
    let task_id = path.into_inner();
    // Using sqlx transaction is better but sequential execution is fine here
    query("DELETE FROM active_timers WHERE task_id = $1").bind(&task_id).execute(pool.get_ref()).await.ok();
    query("DELETE FROM time_entries WHERE task_id = $1").bind(&task_id).execute(pool.get_ref()).await.ok();
    query("DELETE FROM tasks WHERE id = $1").bind(&task_id).execute(pool.get_ref()).await.ok();
    
    HttpResponse::Ok().json(serde_json::json!({"deleted": true}))
}

#[derive(serde::Deserialize)]
pub struct BulkDeleteRequest {
    pub ids: Vec<String>,
}

pub async fn delete_tasks_bulk(pool: web::Data<DbPool>, body: web::Json<BulkDeleteRequest>) -> impl Responder {
    for id in &body.ids {
        query("DELETE FROM active_timers WHERE task_id = $1").bind(id).execute(pool.get_ref()).await.ok();
        query("DELETE FROM time_entries WHERE task_id = $1").bind(id).execute(pool.get_ref()).await.ok();
        query("DELETE FROM tasks WHERE id = $1").bind(id).execute(pool.get_ref()).await.ok();
    }
    HttpResponse::Ok().json(serde_json::json!({"deleted_count": body.ids.len()}))
}

// Add filters
#[derive(serde::Deserialize)]
pub struct TaskFilter {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}
