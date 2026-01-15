# 🚀 Job Hunt Tracker & Performance Case Study
**Live Demo:** [Link to your hosted app on Dokploy]  
**Built by:** Adithep Tamwisai

## 📌 Project Overview
I built this application to solve two problems:
1. **The Organization Problem:** Managing high-volume job applications across multiple platforms while tracking specific CV versions and cover letters sent.
2. **The "Human Proof" Problem:** In an era of AI-generated resumes, this project serves as a "Living Case Study" to prove my ability to ship high-performance, GDPR-compliant Rust code.

## 🛠 Tech Stack & Architecture
- **Frontend:** **Dioxus** (Fullstack Rust) for type-safety and performance.
- **Backend:** **Axum** + **Tokio** to handle concurrent requests with minimal overhead.
- **Database:** **PostgreSQL** (Self-hosted on **Dokploy**).
- **Analytics:** **Umami** (Privacy-focused, no-cookie tracking).



## ⚡ Performance Engineering (99+ Lighthouse)
A primary goal was to validate my expertise in performance optimization. 
- **SSR (Server Side Rendering):** Used to ensure near-instant First Contentful Paint.
- **Zero-Dependency Core:** Minimized JavaScript overhead to maintain an elite **99+ Lighthouse score** across all categories.

## 🛡 Privacy & Compliance (EU/GDPR)
Operating in Finland, I prioritized legal compliance without sacrificing UX.
- **Anonymous Tracking:** The "First Visitor" greeting and visitor counter use server-side salt-hashing of IP addresses. No Personal Identifiable Information (PII) is ever stored.
- **No-Cookie Design:** This site operates legally without an intrusive cookie banner, improving the recruiter's experience.

## 🔑 Admin Dashboard (The "Me" View)
Behind a secure login (Argon2 hashing), I can:
- **Log Applications:** Track company, role, salary, and contact person.
- **Version Control:** Log which specific CV (e.g., *Rust-Focused* vs *General-EN*) was sent to which company.
- **Status Tracking:** A real-time kanban view of my application lifecycle (Applied → Interview → Offer).

## 💡 Key Learnings
- **Infrastructure as Code:** Deploying via **Dokploy/Nixpacks** taught me how to manage a self-hosted PaaS with 1% idle CPU usage.
- **Dioxus Ecosystem:** Gained deep experience in Rust's growing frontend ecosystem and state management.