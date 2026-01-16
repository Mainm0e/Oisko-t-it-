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
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize Theme
    use_effect(move || {
        spawn(async move {
            let _ = document::eval(
                r#"
                const theme = localStorage.getItem('theme') || 'dark';
                document.documentElement.setAttribute('data-theme', theme);
            "#,
            )
            .await;
        });
    });

    rsx! {
        document::Title { "Oisko töitä" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let applications = use_resource(move || async move {
        crate::services::application_service::get_public_applications().await
    });

    rsx! {
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20 relative min-h-screen scanline",
            // Header
            div { class: "flex flex-col items-center justify-center mb-24 relative",
                div { class: "absolute -top-10 w-64 h-64 bg-accent-glow blur-[100px] opacity-20 -z-10" }
                h1 {
                    class: "text-6xl md:text-8xl font-black tracking-tighter text-center mb-6 animate-pulse",
                    style: "color: var(--text-color); text-shadow: 0 0 20px var(--accent-glow);",
                    "OISKO TÖITÄ"
                }
                div { class: "flex items-center gap-4 text-xs tracking-[0.4em] uppercase font-bold opacity-60",
                    span { class: "w-2 h-2 rounded-full bg-green-500 animate-pulse" }
                    "// NEURAL LINK: STABLE"
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
                        div { class: "flex flex-wrap items-center justify-between mb-16 border-y border-white/5 py-8 glass px-8 rounded-xl gap-8 relative",
                            div { class: "flex flex-wrap gap-8 md:gap-16",
                                div { class: "flex flex-col gap-1",
                                    span { class: "text-[10px] uppercase tracking-[0.2em] font-black text-accent-color", "Active Ops" }
                                    span { class: "text-3xl font-black", "{active_ops}" }
                                }
                                div { class: "flex flex-col gap-1",
                                    span { class: "text-[10px] uppercase tracking-[0.2em] font-black opacity-40", "Total Logs" }
                                    span { class: "text-3xl font-black opacity-80", "{total_nodes}" }
                                }
                                div { class: "flex flex-col gap-1",
                                    span { class: "text-[10px] uppercase tracking-[0.2em] font-black text-green-500", "Success Fact" }
                                    span { class: "text-3xl font-black text-green-400", "{success_rate}%" }
                                }
                                div { class: "flex flex-col gap-1",
                                    span { class: "text-[10px] uppercase tracking-[0.2em] font-black text-sky-500", "Sector Depth" }
                                    span { class: "text-3xl font-black text-sky-400", "{sector_diversity}" }
                                }
                            }

                            div { class: "hidden lg:flex flex-col items-end gap-3",
                                h2 { class: "text-[10px] font-black tracking-[0.6em] opacity-40", "GLOBAL_MISSION_LOG" }
                                div { class: "w-64 h-1.5 bg-white/5 rounded-full overflow-hidden border border-white/5 relative",
                                    div {
                                        class: "h-full bg-accent-color shadow-[0_0_20px_var(--accent-glow)] transition-all duration-1000",
                                        style: "width: {success_rate}%"
                                    }
                                }
                            }
                        }

                        if apps.is_empty() {
                            div { class: "text-center py-40 glass rounded-xl border-dashed border-2 border-white/5",
                                p { class: "font-mono uppercase tracking-[0.5em] opacity-20 text-xl", "NO DATA PERSISTED IN CURRENT SECTOR." }
                            }
                        } else {
                            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-10",
                                for app in apps {
                                    {
                                        let date_str = app.created_at.format("%Y.%m.%d / %H:%M").to_string();
                                        rsx! {
                                            Link {
                                                to: Route::ApplicationDetail { id: app.id.to_string() },
                                                class: "noir-card group bg-[var(--card-bg)] block no-underline rounded-sm border-white/5 overflow-hidden",

                                                // Card Decoration
                                                div { class: "absolute top-0 right-0 w-16 h-16 bg-gradient-to-bl from-white/5 to-transparent pointer-events-none" }

                                                div { class: "p-8",
                                                    div { class: "flex justify-between items-start mb-8",
                                                        div { class: "flex items-start gap-4",
                                                            if let Some(logo) = &app.logo_url {
                                                                img {
                                                                    src: "{logo}",
                                                                    class: "w-14 h-14 rounded-lg bg-white/5 object-contain border border-white/10 p-1 group-hover:border-accent-color/50 transition-colors",
                                                                }
                                                            } else {
                                                                div { class: "w-14 h-14 rounded-lg bg-white/5 border border-white/10 flex items-center justify-center text-xl font-bold opacity-30",
                                                                    "?"
                                                                }
                                                            }
                                                            div {
                                                                h3 { class: "text-2xl font-bold mb-1 group-hover:text-accent-color transition-colors", "{app.company}" }
                                                                if let Some(website) = &app.company_website {
                                                                    a {
                                                                        href: "{website}",
                                                                        target: "_blank",
                                                                        class: "text-[10px] uppercase tracking-widest hover:text-white transition-colors z-20 relative inline-flex items-center gap-1 opacity-50",
                                                                        onclick: move |e: Event<MouseData>| e.stop_propagation(),
                                                                        "EXT_CMD_SRC"
                                                                        span { class: "text-[8px]", "↗" }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }

                                                    div { class: "space-y-6 pt-4 border-t border-white/5",
                                                        div {
                                                            div { class: "text-[10px] uppercase tracking-[0.2em] opacity-40 mb-1", "Designation" }
                                                            div { class: "text-lg font-medium", "{app.role}" }
                                                        }

                                                        div { class: "flex justify-between items-end",
                                                            div {
                                                                div { class: "text-[10px] uppercase tracking-[0.2em] opacity-40 mb-1", "Timestamp" }
                                                                div { class: "font-mono text-sm opacity-70", "{date_str}" }
                                                            }

                                                            div {
                                                                class: "px-4 py-1.5 text-[10px] font-black uppercase tracking-[0.3em] border skew-x-[-15deg] transition-all",
                                                                style: match app.status.as_str() {
                                                                    "Offer" => "background: var(--status-offer); color: black; border-color: var(--status-offer); box-shadow: 0 0 15px rgba(16, 185, 129, 0.4);",
                                                                    "Rejected" => "background: transparent; color: var(--status-rejected); border-color: var(--status-rejected); opacity: 0.6;",
                                                                    "Interviewing" => "background: var(--status-interview); color: black; border-color: var(--status-interview); box-shadow: 0 0 15px rgba(14, 165, 233, 0.4);",
                                                                    _ => "background: rgba(255,255,255,0.05); color: white; border-color: white/20;",
                                                                },
                                                                div { class: "skew-x-[15deg]", "{app.status}" }
                                                            }
                                                        }
                                                    }
                                                }

                                                // Footer Interactive Element
                                                div { class: "h-1 w-0 group-hover:w-full bg-accent-color transition-all duration-500 ease-out" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { class: "text-center py-20 border border-red-900/50 bg-red-950/20 rounded-xl",
                        h3 { class: "text-red-500 mb-2 font-black", "CRITICAL SYSTEM ERROR" }
                        p { class: "font-mono text-sm opacity-60", "{e}" }
                    }
                },
                None => rsx! {
                    div { class: "flex flex-col items-center justify-center py-40 gap-6",
                        div { class: "animate-spin rounded-full h-16 w-16 border-t-2 border-b-2 border-accent-color shadow-[0_0_20px_var(--accent-glow)]" }
                        p { class: "text-xs font-black uppercase tracking-[0.6em] animate-pulse opacity-40", "Synchronizing mission log..." }
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
    let mut theme = use_signal(|| "dark".to_string());

    // Sync theme on mount
    use_effect(move || {
        spawn(async move {
            if let Ok(val) = document::eval("return localStorage.getItem('theme') || 'dark'")
                .recv::<String>()
                .await
            {
                theme.set(val);
            }
        });
    });

    rsx! {
        div {
            class: "fixed top-0 left-0 w-full z-50 flex justify-between items-center p-6 mix-blend-difference",
            style: "color: white;", // Always white because of mix-blend-difference

            nav { class: "flex gap-6",
                Link { to: Route::Home {}, class: "hover:underline uppercase tracking-wide font-bold", "Home" }
                // Link { to: Route::Blog { id: 1 }, class: "hover:underline uppercase tracking-wide", "Blog" }
            }

            button {
                class: "px-4 py-1 rounded border border-white hover:bg-white hover:text-black transition-colors cursor-pointer font-bold tracking-widest text-xs uppercase",
                onclick: move |_| {
                    let new_theme = if theme() == "dark" { "light" } else { "dark" };
                    theme.set(new_theme.to_string());
                    spawn(async move {
                        let _ = document::eval(&format!(r#"
                            document.documentElement.setAttribute('data-theme', '{}');
                            localStorage.setItem('theme', '{}');
                        "#, new_theme, new_theme)).await;
                    });
                },
                if theme() == "dark" { "LIGHT" } else { "DARK" }
            }
        }

        Outlet::<Route> {}
    }
}
