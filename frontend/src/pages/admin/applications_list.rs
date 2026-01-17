use crate::components::kanban_board::KanbanBoard;
use crate::models::application::UpdateApplicationPayload;
use dioxus::prelude::*;

#[component]
pub fn ApplicationsList() -> Element {
    let mut view_mode = use_signal(|| "table".to_string());
    let applications = use_resource(move || async move {
        crate::services::application_service::list_applications().await
    });

    let restart_apps = applications.clone();

    let on_status_change = move |(id, new_status): (String, String)| {
        let mut restart_apps = restart_apps.clone();
        async move {
            let payload = UpdateApplicationPayload {
                status: Some(new_status),
                ..Default::default()
            };
            if let Ok(_) =
                crate::services::application_service::update_application(&id, payload).await
            {
                restart_apps.restart();
            }
        }
    };

    rsx! {
        div { class: "max-w-7xl mx-auto space-y-8",
            div { class: "flex flex-col md:flex-row justify-between items-start md:items-center gap-6 border-b pb-8",
                style: "border-color: var(--glass-border);",
                div {
                    h2 { class: "text-4xl font-black tracking-tighter uppercase",
                        style: "color: var(--text-color); text-shadow: 0 0 10px var(--accent-glow);",
                        "Applications"
                    }
                    p { class: "text-[10px] font-mono mt-2 uppercase tracking-[0.2em] opacity-40",
                        style: "color: var(--accent-color)",
                        ":: Tracking and managing active search missions ::"
                    }
                }

                div { class: "flex items-center gap-4",
                    // View Toggle
                    div { class: "flex glass rounded p-1 border",
                        style: "border-color: var(--glass-border);",
                        button {
                            class: "px-4 py-1.5 rounded text-[10px] font-black uppercase tracking-widest transition-all",
                            style: if view_mode() == "table" {
                                "background: var(--accent-color); color: white; box-shadow: 0 0 10px var(--accent-glow);"
                            } else {
                                "color: var(--text-color); opacity: 0.4;"
                            },
                            onclick: move |_| view_mode.set("table".to_string()),
                            "Table"
                        }
                        button {
                            class: "px-4 py-1.5 rounded text-[10px] font-black uppercase tracking-widest transition-all",
                            style: if view_mode() == "board" {
                                "background: var(--accent-color); color: white; box-shadow: 0 0 10px var(--accent-glow);"
                            } else {
                                "color: var(--text-color); opacity: 0.4;"
                            },
                            onclick: move |_| view_mode.set("board".to_string()),
                            "Board"
                        }
                    }

                    Link {
                        to: "/admin/applications/new",
                        class: "noir-btn px-6 py-2.5 text-[10px]",
                        "Add Application"
                    }
                }
            }

            match &*applications.read() {
                Some(Ok(apps)) => rsx! {
                    if view_mode() == "table" {
                        div { class: "noir-card rounded overflow-hidden",
                            div { class: "overflow-x-auto",
                                table { class: "w-full text-left text-xs",
                                    thead { class: "bg-white/5 font-black uppercase tracking-widest text-[10px]",
                                        style: "color: var(--accent-color);",
                                        tr {
                                            th { class: "px-8 py-5", "Company" }
                                            th { class: "px-8 py-5", "Role" }
                                            th { class: "px-8 py-5", "Status" }
                                            th { class: "px-8 py-5", "Intel" }
                                            th { class: "px-8 py-5", "Init Date" }
                                            th { class: "px-8 py-5 text-right", "Actions" }
                                        }
                                    }
                                    tbody { class: "divide-y",
                                        style: "divide-color: var(--glass-border);",
                                        for app in apps {
                                            tr { class: "hover:bg-[var(--hover-bg)] transition-colors group",
                                                td { class: "px-8 py-5 font-bold",
                                                    style: "color: var(--text-color)",
                                                    if let Some(website) = &app.company_website {
                                                        a {
                                                            href: "{website}",
                                                            target: "_blank",
                                                            class: "hover:text-[var(--accent-color)] flex items-center gap-2 transition-colors",
                                                            "{app.company}"
                                                            span { class: "text-[8px] opacity-0 group-hover:opacity-40 transition-opacity", "↗" }
                                                        }
                                                    } else {
                                                        "{app.company}"
                                                    }
                                                }
                                                td { class: "px-8 py-5 opacity-60", "{app.role}" }
                                                td { class: "px-8 py-5",
                                                    span {
                                                        class: "px-3 py-1 rounded text-[8px] font-black uppercase tracking-widest border",
                                                        style: match app.status.as_str() {
                                                            "Offer" | "Accepted" => "background: var(--status-offer); color: white; border-color: var(--status-offer);",
                                                            "Rejected" => "background: var(--status-rejected); color: white; border-color: var(--status-rejected);",
                                                            "Interviewing" => "background: var(--status-interview); color: white; border-color: var(--status-interview);",
                                                            _ => "background: var(--hover-bg); color: var(--text-color); border-color: var(--glass-border);",
                                                        },
                                                        "{app.status}"
                                                    }
                                                }
                                                td { class: "px-8 py-5",
                                                    div { class: "flex flex-col gap-1 text-[8px] font-black tracking-widest uppercase",
                                                        if let Some(cv_path) = &app.cv_path {
                                                            a {
                                                                href: "{BASE_URL}{cv_path}",
                                                                target: "_blank",
                                                                style: "color: var(--accent-color)",
                                                                class: "hover:opacity-70 flex items-center gap-1 transition-all",
                                                                "📄 CV {app.cv_version.as_deref().unwrap_or(\"\")}"
                                                            }
                                                        }
                                                        if let Some(cl_path) = &app.cover_letter_path {
                                                            a {
                                                                href: "{BASE_URL}{cl_path}",
                                                                target: "_blank",
                                                                style: "color: var(--accent-color)",
                                                                class: "hover:opacity-70 flex items-center gap-1 transition-all",
                                                                "📝 Cover Letter"
                                                            }
                                                        }
                                                    }
                                                }
                                                td { class: "px-8 py-5 font-mono opacity-40", "{app.created_at.format(\"%Y.%m.%d\")}" }
                                                td { class: "px-8 py-5 text-right",
                                                    div { class: "flex justify-end gap-4",
                                                        Link {
                                                            to: format!("/admin/applications/{}/edit", app.id),
                                                            class: "text-[10px] font-black tracking-widest uppercase hover:opacity-100 opacity-60 transition-all",
                                                            style: "color: var(--accent-color)",
                                                            "Edit"
                                                        }
                                                        button {
                                                            class: "text-[10px] font-black tracking-widest uppercase hover:opacity-100 opacity-60 transition-all",
                                                            style: "color: var(--status-rejected)",
                                                            onclick: {
                                                                let id = app.id.clone();
                                                                let restart_apps = restart_apps.clone();
                                                                move |_| {
                                                                    let id = id.clone();
                                                                    let mut restart_apps = restart_apps.clone();
                                                                    async move {
                                                                        if let Ok(_) = crate::services::application_service::delete_application(&id.to_string()).await {
                                                                            restart_apps.restart();
                                                                        }
                                                                    }
                                                                }
                                                            },
                                                            "Delete"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div { class: "glass p-8 rounded border",
                            style: "border-color: var(--glass-border); background: var(--card-bg);",
                            KanbanBoard {
                                applications: apps.clone(),
                                on_status_change: on_status_change.clone()
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! { div { class: "text-center py-20 noir-card", p { class: "text-red-500 font-black", "CRITICAL ERROR: {e}" } } },
                None => rsx! { div { class: "text-center py-20 flex flex-col items-center gap-4",
                    div { class: "animate-spin w-8 h-8 border-t-2 border-b-2 border-accent-color rounded-full" }
                    p { class: "text-[10px] font-black uppercase tracking-[0.5em] opacity-40 animate-pulse", "Syncing missions..." }
                } },
            }
        }
    }
}

const BASE_URL: &str = crate::services::application_service::BASE_URL;
