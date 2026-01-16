use dioxus::prelude::*;

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

/// Home page
#[component]
fn Home() -> Element {
    let applications = use_resource(move || async move {
        crate::services::application_service::get_public_applications().await
    });

    rsx! {
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20 relative min-h-screen",
            // Simple Header
            div { class: "flex flex-col items-center justify-center mb-20",
                 h1 {
                    class: "text-5xl md:text-7xl font-extrabold tracking-widest text-center mb-4 transition-colors duration-300",
                    style: "font-family: 'Orbitron', sans-serif; color: var(--text-color);",
                    "OISKO TÖITÄ"
                }
                p {
                    class: "text-lg tracking-[0.2em] uppercase font-light transition-colors duration-300",
                    style: "color: var(--text-color); opacity: 0.7;",
                    "// CAREER PROTOCOL: ACTIVE"
                }
            }

            div { class: "flex items-center justify-center mb-12",
                div { class: "h-px w-24 bg-gray-500/50" }
                h2 {
                    class: "text-2xl font-bold mx-6 tracking-[0.3em] uppercase transition-colors duration-300",
                    style: "color: var(--text-color);",
                     "MISSION LOG"
                }
                div { class: "h-px w-24 bg-gray-500/50" }
            }

            match &*applications.read() {
                Some(Ok(apps)) => rsx! {
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                        for app in apps {
                            Link {
                                to: Route::ApplicationDetail { id: app.id.to_string() },
                                class: "noir-card p-6 relative group bg-[var(--card-bg)] block no-underline",
                                div { class: "flex justify-between items-start mb-6",
                                    div {
                                        h3 { class: "text-xl font-bold mb-2 transition-colors duration-300", style: "color: var(--text-color);", "{app.company}" }
                                        if let Some(website) = &app.company_website {
                                            // Since this is inside a Link, nested a tags might be tricky but valid HTML5 if interactive content isn't nested.
                                            // Actually Link renders as 'a'. Nested 'a' is invalid.
                                            // We should probably use object/embed trick or just make the whole card clickable EXCEPT the external link?
                                            // Dioxus Link handles internal routing.
                                            // For simplicity, let's keep the card clickable and if they want external link they can right click or we style it as button.
                                            // Or we can use an 'object' tag for the internal link to isolate it? No that's complex.
                                            // Let's just render the text for now, or prevent default on the external link click.
                                            // Dioxus `Link` might intercept clicks.

                                            // Alternative: Make the external link a "button" with event propagation stopped?
                                            // Let's simply display the text "OFFICIAL LINK" but maybe not clickable OR accept that clicking it might trigger the card route too.
                                            // Actually, `onclick: stop_propagation` works for nested elements.
                                            // Let's use `a` with `onclick: move |e| e.stop_propagation()`
                                            a {
                                                href: "{website}",
                                                target: "_blank",
                                                class: "text-xs uppercase tracking-widest hover:underline transition-colors duration-300 z-10 relative",
                                                style: "color: var(--text-color); opacity: 0.6;",
                                                onclick: move |e: Event<MouseData>| e.stop_propagation(),
                                                "LINK_↗"
                                            }
                                        }
                                    }
                                    span {
                                        class: "px-2 py-1 text-[10px] font-bold uppercase tracking-widest border transition-colors duration-300",
                                        style: match app.status.as_str() {
                                            "Offer" => "color: var(--text-color); border-color: var(--text-color);",
                                            "Rejected" => "color: var(--text-color); border-color: var(--border-color); text-decoration: line-through;",
                                            _ => "color: var(--text-color); border-color: var(--border-color); opacity: 0.8;",
                                        },
                                        "{app.status}"
                                    }
                                }

                                div { class: "space-y-3",
                                    div { class: "flex items-baseline",
                                        span { class: "text-xs uppercase tracking-widest w-16 opacity-50", style: "color: var(--text-color);", "Role" }
                                        span { class: "font-medium transition-colors duration-300", style: "color: var(--text-color);", "{app.role}" }
                                    }
                                    div { class: "flex items-baseline",
                                        span { class: "text-xs uppercase tracking-widest w-16 opacity-50", style: "color: var(--text-color);", "Date" }
                                        span { class: "font-mono text-sm opacity-80 transition-colors duration-300", style: "color: var(--text-color);", "{app.created_at.to_string().split(' ').next().unwrap_or(&app.created_at.to_string())}" }
                                    }
                                }
                            }
                        }
                    }
                    if apps.is_empty() {
                         div { class: "text-center py-12 font-mono uppercase tracking-widest opacity-50", style: "color: var(--text-color);", "NO DATA FOUND." }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { class: "text-center py-8 font-mono text-red-500", "SYSTEM ERROR: {e}" }
                },
                None => rsx! {
                    div { class: "flex justify-center py-20",
                        div { class: "animate-spin rounded-full h-8 w-8 border-t-2 border-b-2", style: "border-color: var(--text-color);" }
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
