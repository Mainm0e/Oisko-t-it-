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

    rsx! {
        div { class: "flex h-screen bg-[#050510] text-gray-300 font-sans overflow-hidden",
            // Sidebar
            aside { class: "w-72 bg-black/40 backdrop-blur-xl border-r border-cyan-500/20 flex flex-col fixed h-full z-20 shadow-[0_0_20px_rgba(0,0,0,0.5)]",
                // Header
                div { class: "p-8 border-b border-cyan-500/10 relative overflow-hidden group",
                    div { class: "absolute inset-0 bg-cyan-500/5 translate-y-[-100%] group-hover:translate-y-0 transition-transform duration-500 ease-in-out" }
                    h1 {
                        class: "text-2xl font-bold text-white tracking-widest relative z-10",
                        style: "font-family: 'Orbitron', sans-serif; text-shadow: 0 0 10px rgba(0,243,255,0.3);",
                        "COMMAND CTR"
                    }
                    p { class: "text-xs text-cyan-500/60 mt-2 font-mono tracking-wider", ":: OISKO TÖITÄ SYSTEMS ::" }
                }

                nav { class: "flex-1 p-6 space-y-4",
                    Link {
                        to: "/admin/dashboard",
                        class: "block px-4 py-3 rounded border border-transparent hover:border-cyan-500/30 hover:bg-cyan-500/5 text-gray-400 hover:text-cyan-300 transition-all duration-300 group",
                        div { class: "flex items-center gap-3",
                            span { class: "text-xl group-hover:drop-shadow-[0_0_5px_rgba(0,243,255,0.5)]", "📊" }
                            span { class: "font-medium tracking-wide", "Dashboard" }
                        }
                    }

                    Link {
                        to: "/admin/applications",
                        class: "block px-4 py-3 rounded border border-transparent hover:border-purple-500/30 hover:bg-purple-500/5 text-gray-400 hover:text-purple-300 transition-all duration-300 group",
                        div { class: "flex items-center gap-3",
                            span { class: "text-xl group-hover:drop-shadow-[0_0_5px_rgba(188,19,254,0.5)]", "📁" }
                            span { class: "font-medium tracking-wide", "Applications" }
                        }
                    }
                }

                div { class: "p-6 border-t border-cyan-500/10",
                    button {
                        class: "w-full text-left px-4 py-3 rounded border border-red-500/0 hover:border-red-500/30 hover:bg-red-500/5 text-red-400/70 hover:text-red-400 transition-all duration-300 flex items-center gap-3 group",
                        onclick: move |_| {
                            spawn(async move {
                                let _ = document::eval("localStorage.removeItem('admin_token')").await;
                                navigator.push("/admin/login");
                            });
                        },
                        span { class: "text-xl group-hover:rotate-90 transition-transform duration-300", "🔌" }
                        span { class: "font-mono text-sm tracking-widest", "DISCONNECT" }
                    }
                }
            }

            // Main Content Area
            main { class: "flex-1 ml-72 p-8 overflow-y-auto relative",
                // Grid Background Layer
                div { class: "absolute inset-0 bg-[linear-gradient(rgba(0,243,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(0,243,255,0.02)_1px,transparent_1px)] bg-[size:50px_50px] pointer-events-none" }

                div { class: "relative z-10 max-w-7xl mx-auto",
                    Outlet::<crate::Route> {}
                }
            }
        }
    }
}
