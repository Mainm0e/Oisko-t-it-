# Oisko töitä (Job Hunt Command Center)

**Oisko töitä** is a high-performance, futuristic job application tracking platform built with Rust. It transforms the often tedious process of job searching into a tactical operation, featuring a "Command Center" aesthetic and public-facing dossiers for transparent career updates.

## 🛰 Tactical Overview

- **Frontend**: [Dioxus](https://dioxuslabs.com/) (Rust WASM) with Tailwind CSS.
- **Backend**: [Axum](https://github.com/tokio-rs/axum) (Rust) with [SQLx](https://github.com/launchbadge/sqlx) & PostgreSQL.
- **Aesthetic**: "Monochrome Noir / Command Center" – dark mode by default, glassmorphism, and neon accents.

## 🛠 Active Features

### 1. Unified Command Center (Admin Dashboard)
- **Activity Pulse**: Custom SVG visualization of application frequency over 30 days.
- **Logic Distribution**: Donut chart breakdown of application statuses (Offers, Interviews, Rejections).
- **Incoming Transmissions**: Real-time feed of guest comments from public dossiers.
- **Mission Logs**: Full CRUD management of job entries including salary data, cover letters, and files.

### 2. Public Dossiers (Job Showcase)
- **Live Landing Page**: A minimalist, high-end list of active job "missions".
- **Application Dossier**: Detailed view of specific applications (Status, Role, CV download).
- **Communication Channel**: Guest commenting system enabling codename-based interaction.

### 3. Secure Protocol
- **Auth System**: JWT-based authentication with Argon2 password hashing.
- **Data Integrity**: PostgreSQL backend with automated migrations.
- **File Uplink**: Secure multi-part upload system for CVs and tactical documents.

---

## 🚀 Future Reconnaissance (Suggested Features)

To take **Oisko töitä** to the professional level, here are the recommended next steps:

### 1. Real-Time Signal Interception (SSE/WebSockets)
- Currently, you have to refresh to see new comments.
- **Goal**: Implement Server-Sent Events (SSE) so new comments "pop up" on the dashboard instantly with a notification sound/visual.

### 2. Strategic Data Export (PDF/CSV)
- **Goal**: Add a "Generate Report" button to export all application data or a specific dossier into a beautifully formatted PDF for printing or archival.

### 3. Kanban Intelligence
- **Goal**: Add a board view (stages like "Applied", "Interview 1", "Technical", "Offer") where you can drag and drop cards to update their status.

### 4. Automated Company Intelligence
- **Goal**: Integrate with a company information API (like Clearbit or LinkedIn scrapers) to automatically pull company logos and descriptions when you enter a URL.

### 5. Interview Countdown & Reminders
- **Goal**: A dedicated "Upcoming Ops" widget on the dashboard that shows a countdown timer to your next scheduled interview.

### 6. Email Deployment (Mail Integration)
- **Goal**: Allow sending follow-up emails directly from the dashboard using templates, and track if you've sent a "Thank You" note after an interview.

---

## 💻 Tech Stack Depth

- **Backend**: Axum, SQLx, JWT, Argon2, Tower-HTTP (CORS/Tracing).
- **Frontend**: Dioxus 0.6, Reqwest, Serde, Lucide-style SVG components.
- **Database**: PostgreSQL with `sqlx-cli` for migrations.