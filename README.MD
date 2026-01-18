# Oisko töitä (Job Hunt Command Center)

**Oisko töitä** is a high-performance, futuristic job application tracking platform built with Rust. It transforms the often tedious process of job searching into a tactical operation, featuring a "Command Center" aesthetic, real-time intelligence feeds, and interactive dossiers.

## 🛰 Tactical Overview

- **Frontend**: [Dioxus](https://dioxuslabs.com/) (Rust WASM) with Vanilla CSS/Tailwind.
- **Backend**: [Axum](https://github.com/tokio-rs/axum) (Rust) with [SQLx](https://github.com/launchbadge/sqlx) & PostgreSQL.
- **Aesthetic**: "Command Center Noir" – featuring glassmorphism, pulsing glows, and terminal-grade typography.

## 🛠 Active Features

### 1. Central Command Dashboard
- **Mission Statistics**: Real-time calculation of *Active Ops*, *Success Factor*, and *Sector Depth* from live database queries.
- **Tri-Mode OS (Theming)**: Seamless switching between **Tactical Noir** (Dark), **Pristine Light** (Light), and **Bamboo Zen** (Organic) aesthetics with persistent memory.
- **Daily Signal Intercepts**: Real-time visitor tracking and analytics displayed on the main console.

### 2. Scalable Mission Log (Public Interface)
- **High-Performance Search**: Instant client-side search across company names and roles.
- **Status Filter Arrays**: Real-time filtering by application state (Applied, Interviewing, Offer, etc.).
- **Progressive Reveal**: "Reveal More Logs" pagination system designed for 500+ application scale.
- **Comms Channel (SSE)**: Real-time guest commenting system powered by Server-Sent Events for instant feedback.

### 3. Overhauled Admin Suite
- **Mission Control**: Fully restyled admin dashboard featuring glassmorphism and indigo accents.
- **Tactical Kanban Board**: Premium board view for managing application states with hover-glow dossiers.
- **Intelligence Input**: Streamlined "SCAN WEB" utility that automatically extracts company data from URLs.
- **Document Depot**: Secure multi-part upload system for CVs and cover letters.

## 🚀 Future Reconnaissance (Roadmap 2.0)

To achieve maximum dominance in the IT job market, the following strategic updates are planned:

### 1. AI-Powered Mission Briefings (LLM Integration)
- **Goal**: Integrate an LLM to automatically generate tailored cover letters and interview prep notes based on the "scraped" company intel.
- **Impact**: Demonstrates mastery of AI integration and tailored automation.

### 2. Strategic Analytics Dashboard
- **Goal**: Implement advanced metrics like "Time-to-Offer" averages, "Interview Conversion Probabilities", and sector-specific success heatmaps.
- **Impact**: Shows deep data-driven decision-making capabilities.

### 3. Recruiter "Ghost" Access
- **Goal**: Create specialized, read-only "Secure Links" for recruiters that provide a customized view of relevant dossiers and hidden skills metadata.
- **Impact**: Direct, frictionless value delivery to hiring managers.

### 4. GitHub Sync Protocol
- **Goal**: Automatically link specific GitHub repositories or commits to job applications as proof-of-competence for specific roles.
- **Impact**: Closes the loop between application and verifiable coding skill.

---

## 💻 Tech Stack Depth

- **Backend**: Axum, SQLx (PostgreSQL), Tokio (SSE/Broadcast), Argon2.
- **Frontend**: Dioxus 0.7+, Rust-WASM, HTML/CSS/Tailwind.
- **Infrastructure**: Automated migrations, Secure multi-part file handling.

---

## 🛳 Deployment Prep

To launch the Command Center in a live sector:

1. **Backend**:
   - Set `DATABASE_URL` in your environment.
   - Run `cargo run --release`.
   - Ensure port `3000` is exposed.

2. **Frontend**:
   - Set `API_URL` environment variable to your backend domain (e.g., `https://api.yourdomain.com`).
   - Build for production: `dx build --release`.
   - Deploy the generated `dist` folder to your CDN or web server.