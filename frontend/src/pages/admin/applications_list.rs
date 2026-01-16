use dioxus::prelude::*;

#[component]
pub fn ApplicationsList() -> Element {
    let applications = use_resource(move || async move {
        crate::services::application_service::list_applications().await
    });

    let restart_apps = applications.clone();

    rsx! {
        div { class: "max-w-7xl mx-auto",
            div { class: "flex justify-between items-center mb-8",
                h2 { class: "text-3xl font-bold text-white", "Applications" }
                Link {
                    to: "/admin/applications/new",
                    class: "bg-indigo-500 hover:bg-indigo-400 text-white px-4 py-2 rounded-md font-medium transition-colors",
                    "Add Application"
                }
            }

            div { class: "bg-[#161b22] border border-white/10 rounded-xl overflow-hidden",
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
                            match &*applications.read() {
                                Some(Ok(apps)) => rsx! {
                                    for app in apps {
                                        tr { class: "hover:bg-white/5 transition-colors",
                                            td { class: "px-6 py-4 font-medium text-white",
                                                if let Some(website) = &app.company_website {
                                                    a {
                                                        href: "{website}",
                                                        target: "_blank",
                                                        class: "hover:text-indigo-400 hover:underline flex items-center gap-2",
                                                        "{app.company}"
                                                        span { class: "text-gray-500 text-xs", "↗" }
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
                                                        _ => "px-2 py-1 rounded-full text-xs font-medium bg-gray-500/10 text-gray-400 border border-gray-500/20",
                                                    },
                                                    "{app.status}"
                                                }
                                            }
                                            td { class: "px-6 py-4",
                                                div { class: "flex flex-col gap-1 text-xs",
                                                    if let Some(cv_path) = &app.cv_path {
                                                        a {
                                                            href: "http://localhost:3000{cv_path}",
                                                            target: "_blank",
                                                            class: "text-indigo-400 hover:text-indigo-300 flex items-center gap-1",
                                                            "📄 CV {app.cv_version.as_deref().unwrap_or(\"\")}"
                                                        }
                                                    }
                                                    if let Some(cl_path) = &app.cover_letter_path {
                                                        a {
                                                            href: "http://localhost:3000{cl_path}",
                                                            target: "_blank",
                                                            class: "text-indigo-400 hover:text-indigo-300 flex items-center gap-1",
                                                            "📝 Cover Letter"
                                                        }
                                                    }
                                                }
                                            }
                                            td { class: "px-6 py-4", "{app.created_at.format(\"%Y-%m-%d\")}" }
                                            td { class: "px-6 py-4 text-right",
                                                Link {
                                                    to: format!("/admin/applications/{}/edit", app.id),
                                                    class: "text-indigo-400 hover:text-indigo-300 mr-3",
                                                    "Edit"
                                                }
                                                button {
                                                    class: "text-red-400 hover:text-red-300",
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
                                },
                                Some(Err(e)) => rsx! { tr { td { colspan: "6", class: "px-6 py-4 text-center text-red-400", "Error: {e}" } } },
                                None => rsx! { tr { td { colspan: "6", class: "px-6 py-4 text-center", "Loading..." } } },
                            }
                        }
                    }
                }
            }
        }
    }
}
