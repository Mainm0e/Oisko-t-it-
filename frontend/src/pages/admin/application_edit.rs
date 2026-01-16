use crate::models::application::UpdateApplicationPayload;
use dioxus::prelude::*;

#[component]
pub fn ApplicationEdit(id: String) -> Element {
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

    // Fetch existing data
    let _fetch = use_resource(use_reactive(&id, move |id| async move {
        match crate::services::application_service::get_application(&id).await {
            Ok(app) => {
                company.set(app.company);
                company_website.set(app.company_website.unwrap_or_default());
                role.set(app.role);
                status.set(app.status);
                salary.set(app.salary.unwrap_or_default());
                contact_person.set(app.contact_person.unwrap_or_default());
                cv_version.set(app.cv_version.unwrap_or_default());
                cv_path.set(app.cv_path.unwrap_or_default());
                cover_letter.set(app.cover_letter.unwrap_or_default());
                cover_letter_path.set(app.cover_letter_path.unwrap_or_default());
                logo_url.set(app.logo_url.unwrap_or_default());
                description.set(app.description.unwrap_or_default());
            }
            Err(e) => error_msg.set(format!("Failed to load: {}", e)),
        }
    }));

    let update_app = move |_| {
        let id = id.clone();
        async move {
            let payload = UpdateApplicationPayload {
                company: Some(company()),
                company_website: Some(company_website()).filter(|s| !s.is_empty()),
                role: Some(role()),
                status: Some(status()),
                salary: Some(salary()).filter(|s| !s.is_empty()),
                contact_person: Some(contact_person()).filter(|s| !s.is_empty()),
                cv_version: Some(cv_version()).filter(|s| !s.is_empty()),
                cv_path: Some(cv_path()).filter(|s| !s.is_empty()),
                cover_letter: Some(cover_letter()).filter(|s| !s.is_empty()),
                cover_letter_path: Some(cover_letter_path()).filter(|s| !s.is_empty()),
                logo_url: Some(logo_url()).filter(|s| !s.is_empty()),
                description: Some(description()).filter(|s| !s.is_empty()),
            };

            match crate::services::application_service::update_application(&id, payload).await {
                Ok(_) => {
                    navigator.push("/admin/applications");
                }
                Err(e) => {
                    error_msg.set(e);
                }
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
                h2 { class: "text-3xl font-bold text-white", "Edit Application" }
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
                            div { class: "mt-2",
                                input {
                                    r#type: "text",
                                    class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6",
                                    placeholder: "https://...",
                                    value: "{company_website}",
                                    oninput: move |e| company_website.set(e.value())
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
                                        if !cv_path().is_empty() { "Current CV: Uploaded ✓" } else { "Upload CV (PDF/Doc)" }
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
                                    if !cover_letter_path().is_empty() { "Current Cover Letter: Uploaded ✓" } else { "Upload Cover Letter (Optional)" }
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
                            onclick: update_app,
                            disabled: "{uploading}",
                            class: "flex w-full justify-center rounded-md bg-indigo-500 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-400 transition-colors uppercase tracking-wide disabled:opacity-50 disabled:cursor-not-allowed",
                            "Update Application"
                        }
                    }
                }
            }
        }
    }
}
