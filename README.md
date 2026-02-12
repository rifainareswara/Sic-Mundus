# FAI · Time Tracker

**FAI** is a modern, fast, and aesthetic task management and time tracking application. Built with love for team productivity.

## 🚀 Key Features

- **Task Management**: Create, edit, and organize tasks by category and status.
- **Time Tracking**: Built-in timer with easy Start/Stop buttons.
- **Time Logging**: Detailed work history with duration and notes.
- **Visual Dashboard**: Analyze productivity with daily charts and category breakdowns.
- **Premium Design**: Dark mode interface with glassmorphism touches.

## 🛠 Technology Stack

This project is built using a high-performance modern stack:

- **Frontend**: Vue 3, Vite, Tailwind CSS v4, Pinia, Chart.js
- **Backend**: Rust (Actix Web), SQLite, Rusqlite
- **Containerization**: Docker & Docker Compose

## 📦 How to Run

### Using Docker (Recomended)

The easiest way to run the application is with Docker Compose. Ensure Docker is installed.

```bash
docker compose up -d --build
```

The application will be available at:

- **Frontend**: http://localhost:8005
- **Backend**: http://localhost:8006

### Manual Setup (Development)

Prerequisites: Rust (cargo) and Node.js (npm).

1. **Run Backend**:

   ```bash
   cd backend
   cargo run
   ```

   Backend runs on port `8006`.

2. **Run Frontend**:
   ```bash
   cd frontend
   npm install
   npm run dev
   ```
   Frontend runs on port `8005`.

## 📂 Project Structure

```
├── backend/            # Rust Actix Web API
│   ├── src/
│   │   ├── db/         # Database connection & migrations
│   │   ├── handlers/   # API request handlers
│   │   ├── models/     # Data structures
│   │   └── routes.rs   # Route configuration
│   └── time_list.db    # SQLite database (auto-generated)
│
├── frontend/           # Vue 3 Application
│   ├── src/
│   │   ├── api/        # API client
│   │   ├── assets/     # CSS & static assets
│   │   ├── stores/     # Pinia state management
│   │   └── views/      # Vue page components
│   └── vite.config.js  # Vite config (Tailwind & Proxy)
│
├── docker-compose.yml  # Docker orchestration
└── README.md           # This documentation
```

## 📝 API Endpoints

- `GET /api/tasks` — Fetch all tasks
- `POST /api/tasks` — Create new task
- `GET /api/dashboard/summary` — Summary data & charts
- `POST /api/timer/start/{id}` — Start timer for a task
- `POST /api/timer/stop` — Stop active timer

---

_Made with ❤️ by rifai_
