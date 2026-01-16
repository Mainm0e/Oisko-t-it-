use crate::models::application::CreateApplicationPayload;
use dioxus::prelude::*;

#[component]
pub fn ApplicationCreate() -> Element {
    let navigator = use_navigator();
    let mut company = use_signal(|| "".to_string());
    let mut company_website = use_signal(|| "".to_string());
    let mut role = use_signal(|| "".to_string());
    let mut status = use_signal(|| "Applied".to_string());
    let mut salary = use_signal(|| "".to_string());
    let mut contact_person = use_signal(|| "".to_string());
    let mut cv_version = use_signal(|| "".to_string());
    let mut cv_path = use_signal(|| "".to_string());
    let mut cover_letter = use_signal(|| "".to_string());
    let mut cover_letter_path = use_signal(|| "".to_string());
    let mut logo_url = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());
    let mut error_msg = use_signal(|| "".to_string());
    let mut uploading = use_signal(|| false);
    let mut fetching_intel = use_signal(|| false);

    let create_app = move |_| async move {
        let payload = CreateApplicationPayload {
            company: company(),
            company_website: if company_website().is_empty() {
                None
            } else {
                Some(company_website())
            },
            role: role(),
            status: Some(status()),
            salary: if salary().is_empty() {
                None
            } else {
                Some(salary())
            },
            contact_person: if contact_person().is_empty() {
                None
            } else {
                Some(contact_person())
            },
            cv_version: if cv_version().is_empty() {
                None
            } else {
                Some(cv_version())
            },
            cv_path: if cv_path().is_empty() {
                None
            } else {
                Some(cv_path())
            },
            cover_letter: if cover_letter().is_empty() {
                None
            } else {
                Some(cover_letter())
            },
            cover_letter_path: if cover_letter_path().is_empty() {
                None
            } else {
                Some(cover_letter_path())
            },
            logo_url: if logo_url().is_empty() {
                None
            } else {
                Some(logo_url())
            },
            description: if description().is_empty() {
                None
            } else {
                Some(description())
            },
        };

        match crate::services::application_service::create_application(payload).await {
            Ok(_) => {
                navigator.push("/admin/applications");
            }
            Err(e) => {
                error_msg.set(e);
            }
        }
    };

    let upload_handler = move |evt: Event<FormData>, target: &'static str| async move {
        uploading.set(true);
        let files = evt.files();
        if let Some(file) = files.first() {
            let file_name = file.name();
            if let Ok(file_bytes) = file.clone().read_bytes().await {
                match crate::services::application_service::upload_file(
                    file_bytes.to_vec(),
                    file_name,
                )
                .await
                {
                    Ok(url) => {
                        if target == "cv" {
                            cv_path.set(url);
                        } else if target == "cover_letter" {
                            cover_letter_path.set(url);
                        }
                    }
                    Err(e) => error_msg.set(format!("Upload failed: {}", e)),
                }
            }
        }
        uploading.set(false);
    };

    rsx! {
        div { class: "max-w-2xl mx-auto",
            div { class: "flex justify-between items-center mb-8",
                h2 { class: "text-3xl font-bold text-white", "New Application" }
                Link {
                    to: "/admin/applications",
                    class: "text-indigo-400 hover:text-indigo-300 transition-colors",
                    "Back to list"
                }
            }

            div { class: "bg-[#161b22] border border-white/10 rounded-xl p-8 backdrop-blur-sm",
                div { class: "space-y-6",
                    // Basic Info
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        div {
                            label { class: "block text-sm font-medium leading-6 text-gray-300", "Company *" }
                            div { class: "mt-2",
                                input {
                                    r#type: "text",
                                    class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6",
                                    value: "{company}",
                                    oninput: move |e| company.set(e.value())
                                }
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium leading-6 text-gray-300", "Website" }
                            div { class: "mt-2 flex gap-2",
                                input {
                                    r#type: "text",
                                    class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6",
                                    placeholder: "https://...",
                                    value: "{company_website}",
                                    oninput: move |e| company_website.set(e.value())
                                }
                                button {
                                    class: "bg-indigo-600/20 border border-indigo-500/30 text-indigo-400 px-3 rounded-md hover:bg-indigo-600/40 transition-all text-xs font-bold whitespace-nowrap disabled:opacity-50",
                                    disabled: "{fetching_intel}",
                                    onclick: move |_| {
                                        let url = company_website();
                                        if !url.is_empty() {
                                            spawn(async move {
                                                fetching_intel.set(true);
                                                match crate::services::application_service::fetch_company_intel(&url).await {
                                                    Ok(intel) => {
                                                        if let Some(name) = intel.company_name {
                                                            company.set(name);
                                                        }
                                                        if let Some(desc) = intel.description {
                                                            description.set(desc);
                                                        }
                                                        if let Some(logo) = intel.logo_url {
                                                            logo_url.set(logo);
                                                        }
                                                    }
                                                    Err(e) => error_msg.set(format!("Intel fetch failed: {}", e)),
                                                }
                                                fetching_intel.set(false);
                                            });
                                        }
                                    },
                                    if fetching_intel() { "..." } else { "SCAN WEB" }
                                }
                            }
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium leading-6 text-gray-300", "Role *" }
                        div { class: "mt-2",
                            input {
                                r#type: "text",
                                class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6",
                                value: "{role}",
                                oninput: move |e| role.set(e.value())
                            }
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium leading-6 text-gray-300", "Company Description" }
                        div { class: "mt-2",
                            textarea {
                                class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6 min-h-[80px]",
                                placeholder: "Strategic summary of the company...",
                                value: "{description}",
                                oninput: move |e| description.set(e.value())
                            }
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium leading-6 text-gray-300", "Logo URL" }
                        div { class: "mt-2 flex gap-4 items-center",
                            if !logo_url().is_empty() {
                                img {
                                    src: "{logo_url}",
                                    class: "w-10 h-10 rounded bg-white/5 object-contain",
                                }
                            }
                            input {
                                r#type: "text",
                                class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6",
                                placeholder: "https://...",
                                value: "{logo_url}",
                                oninput: move |e| logo_url.set(e.value())
                            }
                        }
                    }

                     div {
                        label { class: "block text-sm font-medium leading-6 text-gray-300", "Status" }
                        div { class: "mt-2",
                            select {
                                class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6",
                                value: "{status}",
                                onchange: move |e| status.set(e.value()),
                                option { value: "Applied", "Applied" }
                                option { value: "Interviewing", "Interviewing" }
                                option { value: "Offer", "Offer" }
                                option { value: "Rejected", "Rejected" }
                                option { value: "Accepted", "Accepted" }
                            }
                        }
                    }

                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        div {
                            label { class: "block text-sm font-medium leading-6 text-gray-300", "Salary" }
                            div { class: "mt-2",
                                input {
                                    r#type: "text",
                                    class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6",
                                    value: "{salary}",
                                    oninput: move |e| salary.set(e.value())
                                }
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium leading-6 text-gray-300", "Contact Person" }
                            div { class: "mt-2",
                                input {
                                    r#type: "text",
                                    class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6",
                                    value: "{contact_person}",
                                    oninput: move |e| contact_person.set(e.value())
                                }
                            }
                        }
                    }

                    // Documents Section
                     div { class: "border-t border-white/10 pt-6",
                        h3 { class: "text-lg font-medium text-white mb-4", "Documents" }
                        div { class: "space-y-6",
                            // CV Section
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                                div {
                                    label { class: "block text-sm font-medium leading-6 text-gray-300", "CV Version" }
                                    div { class: "mt-2",
                                        input {
                                            r#type: "text",
                                            class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6",
                                            placeholder: "e.g. v2.1",
                                            value: "{cv_version}",
                                            oninput: move |e| cv_version.set(e.value())
                                        }
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium leading-6 text-gray-300",
                                        if !cv_path().is_empty() { "CV Uploaded ✓" } else { "Upload CV (PDF/Doc)" }
                                    }
                                    div { class: "mt-2",
                                        input {
                                            r#type: "file",
                                            class: "block w-full text-sm text-gray-400 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-semibold file:bg-indigo-600 file:text-white hover:file:bg-indigo-500",
                                            onchange: move |e| upload_handler(e, "cv")
                                        }
                                        if !cv_path().is_empty() {
                                            div { class: "mt-1 text-xs text-green-400 truncate", "{cv_path}" }
                                        }
                                    }
                                }
                            }

                            // Cover Letter Section
                            div {
                                label { class: "block text-sm font-medium leading-6 text-gray-300",
                                    if !cover_letter_path().is_empty() { "Cover Letter Uploaded ✓" } else { "Upload Cover Letter (Optional)" }
                                }
                                div { class: "mt-2",
                                    input {
                                        r#type: "file",
                                        class: "block w-full text-sm text-gray-400 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-semibold file:bg-indigo-600 file:text-white hover:file:bg-indigo-500",
                                        onchange: move |e| upload_handler(e, "cover_letter")
                                    }
                                    if !cover_letter_path().is_empty() {
                                        div { class: "mt-1 text-xs text-green-400 truncate", "{cover_letter_path}" }
                                    }
                                }
                            }

                            div {
                                label { class: "block text-sm font-medium leading-6 text-gray-300", "Cover Letter Text (Optional)" }
                                div { class: "mt-2",
                                    textarea {
                                        class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6 min-h-[100px]",
                                        value: "{cover_letter}",
                                        oninput: move |e| cover_letter.set(e.value())
                                    }
                                }
                            }
                        }
                     }

                    if !error_msg().is_empty() {
                         div {
                            class: "text-red-400 bg-red-900/20 border-red-500/20 text-sm text-center py-2 rounded border",
                            "{error_msg}"
                        }
                    }

                    if uploading() {
                        div { class: "text-indigo-400 text-sm text-center animate-pulse", "Uploading file..." }
                    }

                    div {
                        button {
                            onclick: create_app,
                            disabled: "{uploading}",
                            class: "flex w-full justify-center rounded-md bg-indigo-500 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-400 transition-colors uppercase tracking-wide disabled:opacity-50 disabled:cursor-not-allowed",
                            "Create Application"
                        }
                    }
                }
            }
        }
    }
}
