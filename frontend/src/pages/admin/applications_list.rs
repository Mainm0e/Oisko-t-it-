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
            div { class: "max-w-7xl mx-auto",
                div { class: "flex flex-col md:flex-row justify-between items-start md:items-center gap-4 mb-8",
                    div {
                        h2 { class: "text-3xl font-bold text-white", "Applications" }
                        p { class: "text-gray-500 text-sm mt-1", "Track and manage your job search missions." }
                    }

                    div { class: "flex items-center gap-4",
                        // View Toggle
                        div { class: "flex bg-[#161b22] border border-white/10 rounded-lg p-1",
                            button {
                                class: format!("px-3 py-1 rounded-md text-xs font-medium transition-all {}", if view_mode() == "table" { "bg-indigo-500 text-white shadow-lg" } else { "text-gray-400 hover:text-white" }),
                                onclick: move |_| view_mode.set("table".to_string()),
                                "Table"
                            }
                            button {
                                class: format!("px-3 py-1 rounded-md text-xs font-medium transition-all {}", if view_mode() == "board" { "bg-indigo-500 text-white shadow-lg" } else { "text-gray-400 hover:text-white" }),
                                onclick: move |_| view_mode.set("board".to_string()),
                                "Board"
                            }
                        }

                        Link {
                            to: "/admin/applications/new",
                            class: "bg-indigo-500 hover:bg-indigo-400 text-white px-4 py-2 rounded-md font-medium transition-all shadow-lg hover:shadow-indigo-500/20 text-sm",
                            "Add Application"
                        }
                    }
                }

                match &*applications.read() {
                    Some(Ok(apps)) => rsx! {
                        if view_mode() == "table" {
                            div { class: "bg-[#161b22] border border-white/10 rounded-xl overflow-hidden shadow-2xl",
                                div { class: "overflow-x-auto",
                                    table { class: "w-full text-left text-sm text-gray-400",
                                        thead { class: "bg-white/5 text-xs uppercase font-medium text-gray-300",
                                            tr {
                                                th { class: "px-6 py-4", "Company" }
                                                th { class: "px-6 py-4", "Role" }
                                                th { class: "px-6 py-4", "Status" }
                                                th { class: "px-6 py-4", "Documents" }
                                                th { class: "px-6 py-4", "Applied Date" }
                                                th { class: "px-6 py-4 text-right", "Actions" }
                                            }
                                        }
                                        tbody { class: "divide-y divide-white/10",
                                            for app in apps {
                                                tr { class: "hover:bg-white/5 transition-colors group",
                                                    td { class: "px-6 py-4 font-medium text-white",
                                                        if let Some(website) = &app.company_website {
                                                            a {
                                                                href: "{website}",
                                                                target: "_blank",
                                                                class: "hover:text-indigo-400 hover:underline flex items-center gap-2 transition-colors",
                                                                "{app.company}"
                                                                span { class: "text-gray-500 text-xs opacity-0 group-hover:opacity-100 transition-opacity", "↗" }
                                                            }
                                                        } else {
                                                            "{app.company}"
                                                        }
                                                    }
                                                    td { class: "px-6 py-4", "{app.role}" }
                                                    td { class: "px-6 py-4",
                                                        span {
                                                            class: match app.status.as_str() {
                                                                "Offer" => "px-2 py-1 rounded-full text-xs font-medium bg-emerald-500/10 text-emerald-400 border border-emerald-500/20",
                                                                "Rejected" => "px-2 py-1 rounded-full text-xs font-medium bg-red-500/10 text-red-400 border border-red-500/20",
                                                                "Interviewing" => "px-2 py-1 rounded-full text-xs font-medium bg-blue-500/10 text-blue-400 border border-blue-500/20",
                                                                "Accepted" => "px-2 py-1 rounded-full text-xs font-medium bg-purple-500/10 text-purple-400 border border-purple-500/20",
                                                                _ => "px-2 py-1 rounded-full text-xs font-medium bg-gray-500/10 text-gray-400 border border-gray-500/20",
                                                            },
                                                            "{app.status}"
                                                        }
                                                    }
                                                    td { class: "px-6 py-4",
                                                        div { class: "flex flex-col gap-1 text-xs",
                                                            if let Some(cv_path) = &app.cv_path {
                                                                a {
                                                                    href: "{BASE_URL}{cv_path}",
                                                                    target: "_blank",
                                                                    class: "text-indigo-400 hover:text-indigo-300 flex items-center gap-1 transition-colors",
                                                                    "📄 CV {app.cv_version.as_deref().unwrap_or(\"\")}"
                                                                }
                                                            }
                                                            if let Some(cl_path) = &app.cover_letter_path {
                                                                a {
                                                                    href: "{BASE_URL}{cl_path}",
                                                                    target: "_blank",
                                                                    class: "text-indigo-400 hover:text-indigo-300 flex items-center gap-1 transition-colors",
                                                                    "📝 Cover Letter"
                                                                }
                                                            }
                                                        }
                                                    }
                                                    td { class: "px-6 py-4 whitespace-nowrap", "{app.created_at.format(\"%Y-%m-%d\")}" }
                                                    td { class: "px-6 py-4 text-right whitespace-nowrap",
                                                        Link {
                                                            to: format!("/admin/applications/{}/edit", app.id),
                                                            class: "text-indigo-400 hover:text-indigo-300 mr-3 transition-colors",
                                                            "Edit"
                                                        }
                                                        button {
                                                            class: "text-red-400 hover:text-red-300 transition-colors",
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
                                                            }
    ,
                                                            "Delete"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            KanbanBoard {
                                applications: apps.clone(),
                                on_status_change: on_status_change.clone()
                            }
                        }
                    },
                    Some(Err(e)) => rsx! { div { class: "text-center py-12 bg-[#161b22] border border-white/10 rounded-xl", p { class: "text-red-400", "Error: {e}" } } },
                    None => rsx! { div { class: "text-center py-12", p { class: "text-gray-500 animate-pulse", "Loading missions..." } } },
                }
            }
        }
}

const BASE_URL: &str = crate::services::application_service::BASE_URL;
