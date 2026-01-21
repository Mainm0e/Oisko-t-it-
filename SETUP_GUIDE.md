# 📟 Oisko töitä - Initialization Protocols (Setup Guide)

Follow these protocols to initialize the "Command Center" in your local sector.

## 📋 Prerequisites

Ensure your terminal is equipped with the following toolchains:

1.  **Rust**: [Install Rust](https://www.rust-lang.org/tools/install) (latest stable).
2.  **Dioxus CLI**: Install via cargo: `cargo install dioxus-cli`.
3.  **Docker**: Desktop or Engine for database containerization.
4.  **SQLx CLI**: `cargo install sqlx-cli` (for migrations).

---

## 🏗️ Sector Setup (Installation)

### 1. Clone the Repository

```bash
git clone https://github.com/Mainm0e/Oisko-t-it-.git
cd Oisko-t-it-
```

### 2. Initialize Database (Docker)

Launch the secure PostgreSQL container:

```bash
docker-compose up -d db
```

*Note: The database will be accessible at `localhost:5433` (mapped from 5432).*

### 3. Backend Deployment

Navigate to the backend sector and apply database schema:

```bash
cd backend

# Create .env file (Optional, or rely on defaults)
echo "DATABASE_URL=postgres://admin:password@localhost:5433/oisko_db" > .env

# Run Migrations
sqlx database create
sqlx migrate run

# Initiate Backend Uplink
cargo run
```

*Status: Backend should be listening on `http://localhost:3000`.*

### 4. Frontend Launch

Open a new terminal and navigate to the frontend sector:

```bash
cd frontend

# Initiate Dioxus Development Server
dx serve
```

*Status: Frontend will be accessible at `http://localhost:8080`.*

---

## � Configuration & Secrets (Env Vars)

The system relies on specific environment variables for security and connectivity.

### Backend Configuration
Create a `.env` file in the `backend/` directory:

```bash
# backend/.env

# [REQUIRED] Database Connection
DATABASE_URL=postgres://admin:password@localhost:5433/oisko_db

# [RECOMMENDED] Security
# Secret key for JWT token generation. Change this in production!
JWT_SECRET=your_super_secret_jwt_key_here

# [OPTIONAL] Logging
# RUST_LOG levels: error, warn, info, debug, trace
RUST_LOG=info

# [OPTIONAL] Email Service (Resend.com)
# Required if you want the Contact form to actually send emails.
RESEND_API_KEY=re_123456789
SENDER_EMAIL=onboarding@resend.dev
OWNER_EMAIL=your_email@example.com

# [OPTIONAL] CORS Policy
# Allow requests from this origin (default is http://localhost:8080)
FRONTEND_URL=http://localhost:8080
```

### Frontend Configuration
The frontend is a WASM application, so environment variables are baked in **at build time**.

**Development (dx serve):**
Dioxus automatically handles `API_URL` if you want to override the default `http://127.0.0.1:3000`.

```bash
# Override API URL during dev
API_URL=http://localhost:3000 dx serve
```

**Production (dx build):**
When building for production, ensure you set the variable before running the build command:

```bash
API_URL=https://api.yourdomain.com dx build --release
```

---

## 🛑 Troubleshooting

-   **Database Connection Failed**: Ensure Docker container `oisko_db` is running. Check ports with `docker ps`.
-   **SQLx Error**: Make sure you have `sqlx-cli` installed and `DATABASE_URL` is correct.
-   **Frontend API Error**: Verify the backend is running on port 3000 and CORS is configured (default allows localhost).
