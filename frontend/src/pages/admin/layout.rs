use dioxus::prelude::*;

#[component]
pub fn AdminLayout() -> Element {
    let navigator = use_navigator();
    // Simple auth check via effect
    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval("return localStorage.getItem('admin_token')");
            let res = eval.recv::<serde_json::Value>().await;
            match res {
                Ok(val) if !val.is_null() => {
                    // Valid token exist
                }
                _ => {
                    navigator.push("/admin/login");
                }
            }
        });
    });

    crate::services::sse_service::use_sse();
    let notifications = crate::services::sse_service::NOTIFICATIONS.read();

    rsx! {
        div { class: "flex h-screen font-sans overflow-hidden",
            style: "background: var(--bg-color); color: var(--text-color);",
            // Sidebar
            aside { class: "w-72 glass border-r flex flex-col fixed h-full z-20 shadow-2xl",
                style: "border-color: var(--glass-border); background: var(--card-bg);",

                div { class: "p-8 border-b relative overflow-hidden group",
                    style: "border-color: var(--glass-border);",
                    div { class: "absolute inset-0 translate-y-[-100%] group-hover:translate-y-0 transition-transform duration-500 ease-in-out",
                        style: "background: var(--accent-glow);"
                    }
                    h1 {
                        class: "text-2xl font-bold tracking-widest relative z-10",
                        style: "font-family: var(--font-header); color: var(--text-color); text-shadow: 0 0 10px var(--accent-glow);",
                        "COMMAND CTR"
                    }
                    p { class: "text-[10px] mt-2 font-mono tracking-wider opacity-60",
                        style: "color: var(--accent-color)",
                        ":: OISKO TÖITÄ SYSTEMS ::"
                    }
                }

                nav { class: "flex-1 p-6 space-y-4",
                    Link {
                        to: "/admin/dashboard",
                        class: "block px-4 py-3 rounded border border-transparent hover:bg-[var(--hover-bg)] transition-all duration-300 group",
                        style: "color: var(--text-color);",
                        div { class: "flex items-center gap-3",
                            span { class: "text-xl group-hover:drop-shadow-[0_0_5px_var(--accent-glow)]", "📊" }
                            span { class: "font-medium tracking-wide uppercase text-xs opacity-70 group-hover:opacity-100", "Dashboard" }
                        }
                    }

                    Link {
                        to: "/admin/applications",
                        class: "block px-4 py-3 rounded border border-transparent hover:bg-[var(--hover-bg)] transition-all duration-300 group",
                        style: "color: var(--text-color);",
                        div { class: "flex items-center gap-3",
                            span { class: "text-xl group-hover:drop-shadow-[0_0_5px_var(--accent-glow)]", "📁" }
                            span { class: "font-medium tracking-wide uppercase text-xs opacity-70 group-hover:opacity-100", "Applications" }
                        }
                    }
                }

                div { class: "p-6 border-t",
                    style: "border-color: var(--glass-border);",
                    button {
                        class: "w-full text-left px-4 py-3 rounded border border-transparent hover:bg-red-500/10 transition-all duration-300 flex items-center gap-3 group",
                        style: "color: var(--status-rejected);",
                        onclick: move |_| {
                            spawn(async move {
                                let _ = document::eval("localStorage.removeItem('admin_token')").await;
                                navigator.push("/admin/login");
                            });
                        },
                        span { class: "text-xl group-hover:rotate-90 transition-transform duration-300", "🔌" }
                        span { class: "font-mono text-[10px] tracking-[0.4em] uppercase", "DISCONNECT" }
                    }
                }
            }

            // Main Content Area
            main { class: "flex-1 ml-72 p-8 overflow-y-auto relative scanline",
                // Grid Background Layer
                div {
                    class: "absolute inset-0 pointer-events-none opacity-20",
                    style: "background-image: linear-gradient(var(--border-color) 1px, transparent 1px), linear-gradient(90deg, var(--border-color) 1px, transparent 1px); background-size: 50px 50px;"
                }

                div { class: "relative z-10 max-w-7xl mx-auto",
                    Outlet::<crate::Route> {}
                }
            }

            // Toast Notifications Container
            div { class: "fixed bottom-8 right-8 z-50 flex flex-col gap-4 pointer-events-none",
                for notification in notifications.iter() {
                    {
                        let accent = if notification.type_ == crate::services::sse_service::NotificationType::Comment { "var(--accent-color)" } else { "var(--status-accepted)" };
                        let glow = if notification.type_ == crate::services::sse_service::NotificationType::Comment { "var(--accent-glow)" } else { "rgba(16, 185, 129, 0.2)" };

                        rsx! {
                            div {
                                key: "{notification.id}",
                                class: "pointer-events-auto glass border p-4 rounded shadow-2xl w-80",
                                style: "border-color: {accent}; box-shadow: 0 0 20px {glow};",
                                div { class: "flex items-center justify-between mb-2",
                                    h5 { class: "text-[10px] font-black tracking-widest uppercase",
                                        style: "color: {accent}",
                                        "{notification.title}"
                                    }
                                    span { class: "text-[8px] font-mono opacity-40", ":: SIGNAL INTERCEPTED ::" }
                                }
                                p { class: "text-xs font-medium opacity-80", "{notification.message}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
