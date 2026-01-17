use crate::models::application::Application;
use dioxus::prelude::*;

#[component]
pub fn KanbanBoard(
    applications: Vec<Application>,
    on_status_change: EventHandler<(String, String)>,
) -> Element {
    let stages = vec!["Applied", "Interviewing", "Offer", "Rejected", "Accepted"];

    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-3 lg:grid-cols-5 gap-6",
            for stage in stages {
                KanbanColumn {
                    stage: stage.to_string(),
                    applications: applications.iter().filter(|a| a.status == stage).cloned().collect(),
                    on_status_change: on_status_change.clone()
                }
            }
        }
    }
}

#[component]
fn KanbanColumn(
    stage: String,
    applications: Vec<Application>,
    on_status_change: EventHandler<(String, String)>,
) -> Element {
    let accent_color = match stage.as_str() {
        "Offer" | "Accepted" => "var(--status-offer)",
        "Rejected" => "var(--status-rejected)",
        "Interviewing" => "var(--status-interview)",
        _ => "var(--accent-color)",
    };

    rsx! {
        div { class: "flex flex-col gap-6 min-h-[500px]",
            div { class: "flex items-center justify-between pb-3 border-b",
                style: "border-color: var(--glass-border);",
                h3 { class: "text-[10px] font-black uppercase tracking-[0.2em]",
                    style: "color: {accent_color};",
                    "{stage}"
                }
                span { class: "text-[10px] font-mono px-2 py-0.5 rounded border opacity-40",
                    style: "border-color: var(--glass-border); color: var(--text-color);",
                    "{applications.len()}"
                }
            }

            div { class: "flex-1 space-y-3",
                for app in applications {
                    KanbanCard {
                        app,
                        on_status_change: on_status_change.clone()
                    }
                }
            }
        }
    }
}

#[component]
fn KanbanCard(app: Application, on_status_change: EventHandler<(String, String)>) -> Element {
    let stages = vec!["Applied", "Interviewing", "Offer", "Rejected", "Accepted"];
    let current_index = stages.iter().position(|&s| s == app.status).unwrap_or(0);

    rsx! {
        div {
            class: "glass border p-4 rounded hover:shadow-xl transition-all group cursor-pointer relative overflow-hidden",
            style: "border-color: var(--glass-border); background: var(--hover-bg);",

            div { class: "absolute top-0 left-0 w-1 h-full opacity-0 group-hover:opacity-100 transition-opacity",
                style: "background: var(--accent-color); box-shadow: 0 0 10px var(--accent-glow);"
            }

            div { class: "flex justify-between items-start mb-2",
                h4 { class: "font-black text-xs uppercase tracking-tight group-hover:translate-x-1 transition-transform",
                    style: "color: var(--text-color)",
                    "{app.company}"
                }
                if let Some(website) = &app.company_website {
                    a {
                        href: "{website}",
                        target: "_blank",
                        class: "opacity-40 hover:opacity-100 transition-opacity",
                        style: "color: var(--text-color)",
                        span { class: "text-[10px]", "↗" }
                    }
                }
            }
            p { class: "text-[10px] font-mono mb-4 opacity-60 uppercase tracking-widest",
                style: "color: var(--accent-color)",
                "{app.role}"
            }

            div { class: "flex items-center justify-between text-[8px] font-mono opacity-40 uppercase tracking-widest",
                span { "{app.created_at.format(\"%Y.%m.%d\")}" }
                if let Some(salary) = &app.salary {
                    span { style: "color: var(--status-offer)", "{salary}" }
                }
            }

            // Quick status move controls
            div { class: "absolute -right-1 top-1/2 -translate-y-1/2 flex flex-col gap-2 opacity-0 group-hover:opacity-100 transition-all transform translate-x-4 group-hover:translate-x-0 scale-90 group-hover:scale-100",
                 if current_index > 0 {
                     button {
                         class: "glass border p-1 rounded hover:shadow-lg transition-all",
                         style: "background: var(--card-bg); border-color: var(--glass-border); color: var(--text-color);",
                         title: "Move Back",
                         onclick: {
                             let id = app.id.to_string();
                             let new_status = stages[current_index - 1].to_string();
                             let on_status_change = on_status_change.clone();
                             move |e| {
                                 e.stop_propagation();
                                 on_status_change.call((id.clone(), new_status.clone()));
                             }
                         },
                         "←"
                     }
                 }
                 if current_index < stages.len() - 1 {
                     button {
                         class: "glass border p-1 rounded hover:shadow-lg transition-all",
                         style: "background: var(--card-bg); border-color: var(--glass-border); color: var(--text-color);",
                         title: "Move Forward",
                         onclick: {
                             let id = app.id.to_string();
                             let new_status = stages[current_index + 1].to_string();
                             let on_status_change = on_status_change.clone();
                             move |e| {
                                 e.stop_propagation();
                                 on_status_change.call((id.clone(), new_status.clone()));
                             }
                         },
                         "→"
                     }
                 }
            }
        }
    }
}
