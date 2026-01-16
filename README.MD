# Oisko töitä (Job Hunt Command Center)

**Oisko töitä** is a high-performance, futuristic job application tracking platform built with Rust. It transforms the often tedious process of job searching into a tactical operation, featuring a "Command Center" aesthetic, real-time intelligence feeds, and interactive dossiers.

## 🛰 Tactical Overview

- **Frontend**: [Dioxus](https://dioxuslabs.com/) (Rust WASM) with Vanilla CSS/Tailwind.
- **Backend**: [Axum](https://github.com/tokio-rs/axum) (Rust) with [SQLx](https://github.com/launchbadge/sqlx) & PostgreSQL.
- **Aesthetic**: "Command Center Noir" – featuring glassmorphism, pulsing glows, and terminal-grade typography.

## 🛠 Active Features

### 1. Central Command Dashboard
- **Mission Statistics**: Real-time calculation of *Active Ops*, *Success Factor*, and *Sector Depth* from live database queries.
- **Tactical Kanban Board**: Drag-and-drop interface for managing mission stages (Applied, Interviewing, Offer, etc.).
- **Activity Pulse**: Visual tracking of application frequency and status distribution.

### 2. Intelligence Dossiers (Public Interface)
- **Sector Analysis**: Public-facing landing page showing current "Mission Logs" to the world.
- **Dossier Detail**: High-fidelity view of specific applications including company intel and job designations.
- **Comms Channel (SSE)**: Real-time guest commenting system powered by Server-Sent Events (SSE) for instant signal interception.

### 3. Automated Intel Scraper
- **Neural Link**: Automatically extracts company logos and descriptions from provided URLs to populate dossiers instantly.
- **Encryption Phase**: Secure JWT-based authentication for the Command Center.

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