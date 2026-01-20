use dioxus::prelude::*;
use std::collections::HashSet;

mod components;
mod models;
mod pages;
mod services;

use pages::admin::application_create::ApplicationCreate;
use pages::admin::application_edit::ApplicationEdit;
use pages::admin::applications_list::ApplicationsList;
use pages::admin::dashboard::AdminDashboard;
use pages::admin::layout::AdminLayout;
use pages::admin_login::AdminLogin;
use pages::admin_register::AdminRegister;
use pages::application_detail::ApplicationDetail;
use pages::verify_email::VerifyEmail;

use components::data_counter::DataCounter;
use components::sector_map::SectorMap;
use components::terminal_text::TerminalText;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/applications/:id")]
    ApplicationDetail { id: String },
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[end_layout]
    #[route("/admin/login")]
    AdminLogin {},
    #[route("/admin/register")]
    AdminRegister {},
    #[route("/admin/verify")]
    VerifyEmail {},

    #[layout(AdminLayout)]
        #[route("/admin/dashboard")]
        AdminDashboard {},
        #[route("/admin/applications")]
        ApplicationsList {},
        #[route("/admin/applications/new")]
        ApplicationCreate {},
        #[route("/admin/applications/:id/edit")]
        ApplicationEdit { id: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize Theme (Force Dark)
    use_effect(move || {
        spawn(async move {
            let _ = document::eval("document.documentElement.setAttribute('data-theme', 'dark');")
                .await;
        });
    });

    rsx! {
        document::Title { "Oisko töitä | Job Hunt Command Center" }
        document::Meta { name: "description", content: "Futuristic job application mission control. Track your career operations with tactical precision." }

        // Open Graph
        document::Meta { property: "og:site_name", content: "Oisko töitä" }
        document::Meta { property: "og:title", content: "Oisko töitä | Command Center" }
        document::Meta { property: "og:description", content: "High-performance job application tracking with real-time signal monitoring." }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:image", content: asset!("/assets/og-image.png") }
        document::Meta { property: "og:image:width", content: "1200" }
        document::Meta { property: "og:image:height", content: "630" }

        // Twitter Card
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: "Oisko töitä | Command Center" }
        document::Meta { name: "twitter:description", content: "High-performance job application tracking with real-time signal monitoring." }
        document::Meta { name: "twitter:image", content: asset!("/assets/og-image.png") }

        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Orbitron:wght@400;700;900&family=Quicksand:wght@300;400;500;600;700&family=Rajdhani:wght@300;400;500;600;700&display=swap" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let applications = use_resource(move || async move {
        crate::services::application_service::get_public_applications().await
    });

    let visitor_stats =
        use_resource(
            move || async move { crate::services::application_service::record_visit().await },
        );

    let mut show_acknowledgement = use_signal(|| false);
    let mut acknowledgment_triggered = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());
    let mut status_filter = use_signal(|| "All".to_string());
    let mut visible_count = use_signal(|| 12);

    // Effect to trigger acknowledgement automatically
    use_effect(move || {
        if !acknowledgment_triggered() {
            if let Some(Ok(v)) = &*visitor_stats.read() {
                if v.is_first_of_day {
                    show_acknowledgement.set(true);
                    acknowledgment_triggered.set(true);
                }
            }
        }
    });

    let sys_uid = use_memo(|| uuid::Uuid::new_v4().to_string());
    let sys_time = use_memo(|| {
        let finland_time = chrono::Utc::now() + chrono::Duration::hours(2);
        finland_time.format("%H:%M EET").to_string()
    });

    rsx! {
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20 relative min-h-screen scanline",

            // Tactical Acknowledgement Popup
            if show_acknowledgement() {
                div {
                    class: "fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm animate-in fade-in duration-500",
                    onclick: move |_| show_acknowledgement.set(false),
                    div {
                        class: "relative max-w-lg w-full bg-black border-2 p-8 overflow-hidden group transition-all duration-700",
                        style: "border-color: var(--accent-color); box-shadow: 0 0 50px var(--accent-glow); background: var(--bg-color);",
                        onclick: move |e| e.stop_propagation(),

                        // Decorative Elements
                        div { class: "absolute top-0 left-0 w-full h-1", style: "background: var(--accent-color); box-shadow: 0 0 20px var(--accent-glow);" }
                        div { class: "absolute top-0 right-0 w-16 h-16 rotate-45 translate-x-8 -translate-y-8", style: "background: var(--accent-glow);" }

                        div { class: "flex items-center gap-6 mb-8",
                             div { class: "w-16 h-16 rounded-full border-2 flex items-center justify-center text-3xl", style: "border-color: var(--accent-color)", "🎖️" }
                             div {
                                 h2 { class: "text-2xl font-black tracking-tighter uppercase", style: "color: var(--text-color)", "Tactical Recognition" }
                                 p { class: "text-[10px] font-mono tracking-[0.4em] uppercase opacity-60", style: "color: var(--accent-color)", "// SIGNAL_LEADER_DETECTED" }
                             }
                        }

                        div { class: "space-y-4 mb-10",
                            p { class: "text-sm font-mono leading-relaxed opacity-80",
                                style: "color: var(--text-color)",
                                "Attention. You are the "
                                span { class: "font-bold", style: "color: var(--accent-color)", "FIRST" }
                                " human intelligence factor to interface with this command center today."
                            }
                            p { class: "text-xs font-mono italic opacity-60",
                                style: "color: var(--accent-color)",
                                "Your presence has been indexed. This sector remains stable because of early intercepts like yours."
                            }
                        }

                        button {
                            class: "w-full py-4 text-white font-black uppercase tracking-[0.4em] text-xs hover:bg-white hover:text-black transition-all active:scale-95 duration-200",
                            style: "background: var(--accent-color); box-shadow: 0 0 20px var(--accent-glow);",
                            onclick: move |_| show_acknowledgement.set(false),
                            "ACKNOWLEDGE & PROCEED"
                        }

                        // System Meta
                        div { class: "mt-6 pt-6 border-t border-white/5 flex justify-between items-center text-[8px] font-mono opacity-30 tracking-widest",
                            span { "SYS_UID: {sys_uid().get(0..8).unwrap_or(\"????\")}" }
                            span { "LOC: {sys_time}" }
                        }
                    }
                }
            }

            // Header
            div { class: "flex flex-col items-center justify-center mb-24 relative",
                div { class: "absolute -top-10 w-64 h-64 bg-accent-glow blur-[100px] opacity-20 -z-10" }
                h1 {
                    class: "text-6xl md:text-8xl font-black tracking-tighter text-center mb-6 animate-pulse glitch-text",
                    style: "color: var(--text-color); text-shadow: 0 0 20px var(--accent-glow);",
                    TerminalText {
                        text: "OISKO TÖITÄ".to_string(),
                        speed: 100,
                        decode: true,
                        class: "glitch-text"
                    }
                }
                div { class: "flex items-center gap-4 text-xs tracking-[0.4em] uppercase font-bold opacity-60",
                    span { class: "w-2 h-2 rounded-full bg-green-500 animate-pulse" }
                    TerminalText { text: "// NEURAL LINK: STABLE".to_string(), speed: 50, delay: 1000, cursor: false }
                    span { class: "mx-2 opacity-30", "|" }
                    "EST: 2026.01.16"
                }
            }

            match &*applications.read() {
                Some(Ok(apps)) => {
                    let total_nodes = apps.len();
                    let success_nodes = apps.iter().filter(|a| a.status == "Offer" || a.status == "Accepted").count();
                    let success_rate = if total_nodes > 0 { (success_nodes as f32 / total_nodes as f32 * 100.0) as i32 } else { 0 };
                    let sector_diversity = apps.iter().map(|a| &a.company).collect::<HashSet<_>>().len();
                    let active_ops = apps.iter().filter(|a| a.status == "Applied" || a.status == "Interviewing").count();

                    rsx! {
                        // Stats Bar
                        div { class: "flex flex-wrap items-center justify-between mb-16 border-y border-white/5 py-8 glass px-8 rounded-xl gap-8 relative overflow-hidden",
                            // Sector Map Decoration
                            div { class: "absolute -right-20 -top-20 w-64 opacity-20 group-hover:opacity-40 transition-opacity",
                                SectorMap {}
                            }

                            div { class: "flex flex-wrap gap-8 md:gap-16 relative z-10",
                                div { class: "flex flex-col gap-1",
                                    span { class: "text-[10px] uppercase tracking-[0.2em] font-black text-accent-color", "Active Ops" }
                                    span { class: "text-3xl font-black", DataCounter { value: active_ops as i32 } }
                                }
                                div { class: "flex flex-col gap-1",
                                    span { class: "text-[10px] uppercase tracking-[0.2em] font-black opacity-40", "Total Logs" }
                                    span { class: "text-3xl font-black opacity-80", DataCounter { value: total_nodes as i32 } }
                                }
                                div { class: "flex flex-col gap-1",
                                    span { class: "text-[10px] uppercase tracking-[0.2em] font-black", style: "color: var(--status-offer)", "Success Fact" }
                                    span { class: "text-3xl font-black", style: "color: var(--status-offer)", DataCounter { value: success_rate, suffix: "%".to_string() } }
                                }
                                div { class: "flex flex-col gap-1",
                                    span { class: "text-[10px] uppercase tracking-[0.2em] font-black", style: "color: var(--status-interview)", "Sector Depth" }
                                    span { class: "text-3xl font-black", style: "color: var(--status-interview)", DataCounter { value: sector_diversity as i32 } }
                                }
                                // Visitor Stats
                                {
                                    match &*visitor_stats.read() {
                                        Some(Ok(v)) => rsx! {
                                            div { class: "flex flex-col gap-1 pl-8 border-l border-white/10 relative group",
                                                span { class: "text-[10px] uppercase tracking-[0.2em] font-black animate-pulse", style: "color: var(--accent-color)", "Daily Intercepts" }
                                                div { class: "flex items-center gap-4",
                                                    span { class: "text-3xl font-black", style: "color: var(--accent-color)", DataCounter { value: v.today_visitors as i32 } }
                                                    button {
                                                        class: "text-[10px] px-2 py-1 rounded border transition-all opacity-0 group-hover:opacity-100",
                                                        style: "background: var(--accent-glow); color: var(--accent-color); border-color: var(--glass-border);",
                                                        onclick: move |_| show_acknowledgement.set(true),
                                                        "REPLAY_ACK"
                                                    }
                                                }
                                            }
                                            div { class: "flex flex-col gap-1",
                                                span { class: "text-[10px] uppercase tracking-[0.2em] font-black opacity-40", "Total Signals" }
                                                span { class: "text-3xl font-black opacity-80", DataCounter { value: v.total_unique_visitors as i32 } }
                                            }
                                        },
                                        _ => rsx! {
                                            div { class: "flex flex-col gap-1 pl-8 border-l border-white/10 opacity-30",
                                                span { class: "text-[10px] uppercase tracking-[0.2em] font-black", "SCANNING..." }
                                                span { class: "text-3xl font-black", "0" }
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "hidden lg:flex flex-col items-end gap-3 relative z-10",
                                h2 { class: "text-[10px] font-black tracking-[0.6em] opacity-40", "GLOBAL_MISSION_LOG" }
                                div { class: "w-64 h-1.5 bg-white/5 rounded-full overflow-hidden border border-white/5 relative",
                                    div {
                                        class: "h-full bg-accent-color shadow-[0_0_20px_var(--accent-glow)] transition-all duration-1000",
                                        style: "width: {success_rate}%"
                                    }
                                }
                            }
                        }

                        // Search and Filter Bar
                        div { class: "flex flex-wrap mb-12 gap-4 items-center glass p-6 rounded border",
                            style: "border-color: var(--glass-border);",
                            div { class: "flex-1 min-w-[300px] relative",
                                span { class: "absolute left-4 top-1/2 -translate-y-1/2 opacity-40 text-[10px] font-mono", "SEARCH//" }
                                input {
                                    class: "w-full border rounded px-4 py-4 pl-24 text-xs font-mono focus:border-accent-color outline-none transition-all tracking-[0.2em] uppercase",
                                    style: "background: var(--hover-bg); border-color: var(--glass-border); color: var(--text-color);",
                                    placeholder: "ENTER_COMPANY_OR_ROLE",
                                    value: "{search_query}",
                                    oninput: move |e| search_query.set(e.value())
                                }
                            }
                            div { class: "flex flex-wrap gap-2",
                                for status in ["All", "Applied", "Interviewing", "Offer", "Rejected", "Accepted"] {
                                    {
                                        let is_active = status_filter() == status;
                                        let btn_class = if is_active { "px-4 py-3 text-[10px] font-black uppercase tracking-widest border transition-all filter-btn active" } else { "px-4 py-3 text-[10px] font-black uppercase tracking-widest border transition-all filter-btn" };
                                        rsx! {
                                            button {
                                                class: "{btn_class}",
                                                onclick: move |_| {
                                                    status_filter.set(status.to_string());
                                                    visible_count.set(12);
                                                },
                                                "{status}"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        {
                            let filtered: Vec<_> = apps.iter().filter(|app| {
                                let query = search_query().to_lowercase();
                                let matches_search = app.company.to_lowercase().contains(&query) || app.role.to_lowercase().contains(&query);
                                let matches_status = if status_filter() == "All" { true } else { app.status == status_filter() };
                                matches_search && matches_status
                            }).collect();

                            let total_filtered = filtered.len();
                            let display_apps = filtered.into_iter().take(visible_count()).collect::<Vec<_>>();

                            if total_filtered == 0 {
                                rsx! {
                                    div { class: "text-center py-40 glass rounded-lg border-2 border-dashed border-[var(--glass-border)] group hover:border-[var(--accent-color)] transition-all duration-500",
                                        div { class: "mb-6 text-4xl opacity-20 group-hover:opacity-40 transition-opacity", "📡" }
                                        p { class: "font-mono uppercase tracking-[0.5em] opacity-40 text-sm mb-2", "NO SIGNALS_RECOGNIZED_IN_SECTOR" }
                                        p { class: "text-[10px] font-mono uppercase tracking-[0.2em] opacity-20", "Initialize a new mission via the Secure Admin Terminal." }
                                    }
                                }
                            } else {
                                rsx! {
                                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                                        for app in display_apps {
                                            {
                                                let date_str = app.created_at.format("%Y.%m.%d").to_string();
                                                rsx! {
                                                    Link {
                                                        to: Route::ApplicationDetail { id: app.id.to_string() },
                                                        class: "noir-card group bg-[var(--card-bg)] block no-underline rounded-sm border-white/5 overflow-hidden",
                                                        div { class: "p-8",
                                                            div { class: "flex justify-between items-start mb-8",
                                                                div { class: "flex items-start gap-4",
                                                                    if let Some(logo) = &app.logo_url {
                                                                        img { src: "{logo}", class: "w-12 h-12 rounded bg-white/5 object-contain border border-white/10 p-1" }
                                                                    } else {
                                                                        div { class: "w-12 h-12 rounded bg-white/5 border border-white/10 flex items-center justify-center text-xl font-bold opacity-30", "?" }
                                                                    }
                                                                    div {
                                                                        h3 { class: "text-xl font-bold mb-1 group-hover:text-accent-color transition-colors", "{app.company}" }
                                                                        div { class: "text-[10px] font-mono opacity-30 uppercase tracking-widest", "{app.role}" }
                                                                    }
                                                                }
                                                                div {
                                                                    class: "px-2 py-1 border rounded text-[8px] font-black uppercase tracking-widest transition-colors",
                                                                    style: match app.status.as_str() {
                                                                        "Offer" | "Accepted" => "background: var(--status-offer); color: white; border-color: var(--status-offer);",
                                                                        "Interviewing" => "background: var(--status-interview); color: white; border-color: var(--status-interview);",
                                                                        "Rejected" => "background: var(--status-rejected); color: white; border-color: var(--status-rejected);",
                                                                        _ => "background: rgba(255,255,255,0.05); color: var(--text-color); border-color: var(--border-color);"
                                                                    },
                                                                    "{app.status}"
                                                                }
                                                            }
                                                            div { class: "flex justify-between items-center pt-6 border-t border-white/5",
                                                                span { class: "text-[10px] font-mono opacity-20", "{date_str}" }
                                                                span { class: "text-xl transition-transform group-hover:translate-x-1", style: "color: var(--accent-color)", "→" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    if total_filtered > visible_count() {
                                        div { class: "mt-16 flex justify-center",
                                            button {
                                                class: "noir-btn px-12 py-4 text-[10px] font-black tracking-[0.5em]",
                                                onclick: move |_| visible_count.set(visible_count() + 12),
                                                "REVEAL_MORE_LOGS//"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { class: "text-center py-20 border border-red-900/50 bg-red-950/20 rounded",
                        h3 { class: "text-red-500 mb-2 font-black", "CRITICAL SYSTEM ERROR" }
                        p { class: "font-mono text-xs opacity-60", "{e}" }
                    }
                },
                None => rsx! {
                    div { class: "flex flex-col items-center justify-center py-40 gap-6",
                        div { class: "animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-accent-color" }
                        p { class: "text-[10px] font-black uppercase tracking-[0.5em] animate-pulse opacity-40", "SYNCING_LOGS..." }
                    }
                }
            }
        }
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            class: "fixed top-0 left-0 w-full z-50 flex justify-between items-center p-6 pointer-events-none",

            div { class: "mix-blend-difference pointer-events-auto",
                style: "color: white;",
                nav { class: "flex gap-6" }
            }
        }

        Outlet::<Route> {}
    }
}
