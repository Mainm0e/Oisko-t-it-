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
    let color_class = match stage.as_str() {
        "Offer" => "text-emerald-400 border-emerald-500/20",
        "Rejected" => "text-red-400 border-red-500/20",
        "Interviewing" => "text-blue-400 border-blue-500/20",
        "Accepted" => "text-purple-400 border-purple-500/20",
        _ => "text-gray-400 border-white/10",
    };

    rsx! {
        div { class: "flex flex-col gap-4 min-h-[500px]",
            div { class: "flex items-center justify-between pb-2 border-b border-white/10",
                h3 { class: "text-sm font-bold uppercase tracking-wider {color_class}",
                    "{stage}"
                }
                span { class: "bg-white/5 text-gray-500 text-xs px-2 py-0.5 rounded-full",
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
            class: "bg-[#161b22] border border-white/10 p-4 rounded-lg hover:border-indigo-500/50 transition-all group cursor-pointer relative",
            div { class: "flex justify-between items-start mb-2",
                h4 { class: "font-bold text-white group-hover:text-indigo-400 transition-colors", "{app.company}" }
                if let Some(website) = &app.company_website {
                    a {
                        href: "{website}",
                        target: "_blank",
                        class: "text-gray-500 hover:text-white transition-colors",
                        span { class: "text-xs", "↗" }
                    }
                }
            }
            p { class: "text-sm text-gray-400 mb-3", "{app.role}" }

            div { class: "flex items-center justify-between text-[10px] text-gray-500",
                span { "{app.created_at.format(\"%Y-%m-%d\")}" }
                if let Some(salary) = &app.salary {
                    span { class: "text-emerald-500/70", "{salary}" }
                }
            }

            // Quick status move controls
            div { class: "absolute -right-2 top-1/2 -translate-y-1/2 flex flex-col gap-1 opacity-0 group-hover:opacity-100 transition-all transform translate-x-2 group-hover:translate-x-0",
                 if current_index > 0 {
                     button {
                         class: "bg-[#1c2128] border border-white/10 p-1 rounded-md text-gray-400 hover:text-white hover:border-indigo-500/50 shadow-xl transition-all",
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
                         class: "bg-[#1c2128] border border-white/10 p-1 rounded-md text-gray-400 hover:text-white hover:border-indigo-500/50 shadow-xl transition-all",
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
