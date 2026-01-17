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
            div { class: "max-w-2xl mx-auto space-y-8 pb-12",
                div { class: "flex justify-between items-center border-b pb-6",
                    style: "border-color: var(--glass-border);",
                    h2 { class: "text-4xl font-black tracking-tighter uppercase",
                        style: "color: var(--text-color); text-shadow: 0 0 10px var(--accent-glow);",
                        "New Application"
                    }
                    Link {
                        to: "/admin/applications",
                        class: "text-[10px] font-black uppercase tracking-[0.2em] transition-all opacity-40 hover:opacity-100",
                        style: "color: var(--accent-color)",
                        "<< Back to list"
                    }
                }

                div { class: "noir-card p-8",
                    div { class: "space-y-8",
                        // Basic Info
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                            div {
                                label { class: "block text-[10px] font-black uppercase tracking-[0.2em] mb-2 opacity-60",
                                    style: "color: var(--text-color)",
                                    "Company *"
                                }
                                input {
                                    r#type: "text",
                                    class: "w-full bg-[var(--hover-bg)] border border-[var(--glass-border)] rounded px-4 py-3 text-xs font-mono focus:border-[var(--accent-color)] outline-none transition-all tracking-wider text-white",
                                    value: "{company}",
                                    oninput: move |e| company.set(e.value())
                                }
                            }
                            div {
                                label { class: "block text-[10px] font-black uppercase tracking-[0.2em] mb-2 opacity-60",
                                    style: "color: var(--text-color)",
                                    "Website"
                                }
                                div { class: "flex gap-3",
                                    input {
                                        r#type: "text",
                                        class: "flex-1 bg-[var(--hover-bg)] border border-[var(--glass-border)] rounded px-4 py-3 text-xs font-mono focus:border-[var(--accent-color)] outline-none transition-all tracking-wider text-white",
                                        placeholder: "https://...",
                                        value: "{company_website}",
                                        oninput: move |e| company_website.set(e.value())
                                    }
                                    button {
                                        class: "px-4 rounded text-[8px] font-black uppercase transition-all border disabled:opacity-30",
                                        style: "background: var(--accent-glow); color: var(--accent-color); border-color: var(--accent-color);",
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
                            label { class: "block text-[10px] font-black uppercase tracking-[0.2em] mb-2 opacity-60",
                                style: "color: var(--text-color)",
                                "Role *"
                            }
                            input {
                                r#type: "text",
                                class: "w-full bg-[var(--hover-bg)] border border-[var(--glass-border)] rounded px-4 py-3 text-xs font-mono focus:border-[var(--accent-color)] outline-none transition-all tracking-wider text-white",
                                value: "{role}",
                                oninput: move |e| role.set(e.value())
                            }
                        }

                        div {
                            label { class: "block text-[10px] font-black uppercase tracking-[0.2em] mb-2 opacity-60",
                                style: "color: var(--text-color)",
                                "Company Description"
                            }
                            textarea {
                                class: "w-full bg-[var(--hover-bg)] border border-[var(--glass-border)] rounded px-4 py-3 text-xs font-sans focus:border-[var(--accent-color)] outline-none transition-all tracking-wide text-white min-h-[100px]",
                                placeholder: "Strategic summary of the company...",
                                value: "{description}",
                                oninput: move |e| description.set(e.value())
                            }
                        }

                        div {
                            label { class: "block text-[10px] font-black uppercase tracking-[0.2em] mb-2 opacity-60",
                                style: "color: var(--text-color)",
                                "Logo URL"
                            }
                            div { class: "flex gap-4 items-center",
                                if !logo_url().is_empty() {
                                    img {
                                        src: "{logo_url}",
                                        class: "w-12 h-12 rounded border p-1 bg-white/5 object-contain",
                                        style: "border-color: var(--glass-border);",
                                    }
                                }
                                input {
                                    r#type: "text",
                                    class: "flex-1 bg-[var(--hover-bg)] border border-[var(--glass-border)] rounded px-4 py-3 text-xs font-mono focus:border-[var(--accent-color)] outline-none transition-all tracking-wider text-white",
                                    placeholder: "https://...",
                                    value: "{logo_url}",
                                    oninput: move |e| logo_url.set(e.value())
                                }
                            }
                        }

                         div {
                            label { class: "block text-[10px] font-black uppercase tracking-[0.2em] mb-2 opacity-60",
                                style: "color: var(--text-color)",
                                "Status"
                            }
                            select {
                                class: "w-full bg-[var(--hover-bg)] border border-[var(--glass-border)] rounded px-4 py-3 text-xs font-black uppercase tracking-widest focus:border-[var(--accent-color)] outline-none transition-all text-white",
                                value: "{status}",
                                onchange: move |e| status.set(e.value()),
                                option { value: "Applied", "Applied" }
                                option { value: "Interviewing", "Interviewing" }
                                option { value: "Offer", "Offer" }
                                option { value: "Rejected", "Rejected" }
                                option { value: "Accepted", "Accepted" }
                            }
                        }

                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                            div {
                                label { class: "block text-[10px] font-black uppercase tracking-[0.2em] mb-2 opacity-60",
                                    style: "color: var(--text-color)",
                                    "Salary"
                                }
                                input {
                                    r#type: "text",
                                    class: "w-full bg-[var(--hover-bg)] border border-[var(--glass-border)] rounded px-4 py-3 text-xs font-mono focus:border-[var(--accent-color)] outline-none transition-all tracking-wider text-white",
                                    value: "{salary}",
                                    oninput: move |e| salary.set(e.value())
                                }
                            }
                            div {
                                label { class: "block text-[10px] font-black uppercase tracking-[0.2em] mb-2 opacity-60",
                                    style: "color: var(--text-color)",
                                    "Contact Person"
                                }
                                input {
                                    r#type: "text",
                                    class: "w-full bg-[var(--hover-bg)] border border-[var(--glass-border)] rounded px-4 py-3 text-xs font-mono focus:border-[var(--accent-color)] outline-none transition-all tracking-wider text-white",
                                    value: "{contact_person}",
                                    oninput: move |e| contact_person.set(e.value())
                                }
                            }
                        }

                        // Documents Section
                         div { class: "border-t pt-8",
                            style: "border-color: var(--glass-border);",
                            h3 { class: "text-lg font-black uppercase tracking-widest mb-6 opacity-80",
                                style: "color: var(--text-color)",
                                "Documents"
                            }
                            div { class: "space-y-8",
                                // CV Section
                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                                    div {
                                        label { class: "block text-[10px] font-black uppercase tracking-[0.2em] mb-2 opacity-60",
                                            style: "color: var(--text-color)",
                                            "CV Version"
                                        }
                                        input {
                                            r#type: "text",
                                            class: "w-full bg-[var(--hover-bg)] border border-[var(--glass-border)] rounded px-4 py-3 text-xs font-mono focus:border-[var(--accent-color)] outline-none transition-all tracking-wider text-white",
                                            placeholder: "e.g. v2.1",
                                            value: "{cv_version}",
                                            oninput: move |e| cv_version.set(e.value())
                                        }
                                    }
                                    div {
                                        label { class: "block text-[10px] font-black uppercase tracking-[0.2em] mb-2 opacity-60",
                                            style: "color: var(--text-color)",
                                            if !cv_path().is_empty() { "CV Uploaded ✓" } else { "Upload CV (PDF/Doc)" }
                                        }
                                        input {
                                            r#type: "file",
                                            class: "w-full text-[10px] font-mono opacity-60 file:mr-4 file:py-2 file:px-4 file:rounded file:border file:text-[10px] file:font-black file:uppercase file:tracking-widest file:bg-[var(--hover-bg)] file:text-[var(--accent-color)] file:border-[var(--glass-border)] hover:file:bg-[var(--accent-glow)] transition-all",
                                            onchange: move |e| upload_handler(e, "cv")
                                        }
                                        if !cv_path().is_empty() {
                                            div { class: "mt-2 text-[8px] font-mono truncate opacity-40 uppercase", "{cv_path}" }
                                        }
                                    }
                                }

                                // Cover Letter Section
                                div {
                                    label { class: "block text-[10px] font-black uppercase tracking-[0.2em] mb-2 opacity-60",
                                        style: "color: var(--text-color)",
                                        if !cover_letter_path().is_empty() { "Cover Letter Uploaded ✓" } else { "Upload Cover Letter (Optional)" }
                                    }
                                    input {
                                        r#type: "file",
                                        class: "w-full text-[10px] font-mono opacity-60 file:mr-4 file:py-2 file:px-4 file:rounded file:border file:text-[10px] file:font-black file:uppercase file:tracking-widest file:bg-[var(--hover-bg)] file:text-[var(--accent-color)] file:border-[var(--glass-border)] hover:file:bg-[var(--accent-glow)] transition-all",
                                        onchange: move |e| upload_handler(e, "cover_letter")
                                    }
                                    if !cover_letter_path().is_empty() {
                                        div { class: "mt-2 text-[8px] font-mono truncate opacity-40 uppercase", "{cover_letter_path}" }
                                    }
                                }

                                div {
                                    label { class: "block text-[10px] font-black uppercase tracking-[0.2em] mb-2 opacity-60",
                                        style: "color: var(--text-color)",
                                        "Cover Letter Text (Optional)"
                                    }
                                    textarea {
                                        class: "w-full bg-[var(--hover-bg)] border border-[var(--glass-border)] rounded px-4 py-3 text-xs font-sans focus:border-[var(--accent-color)] outline-none transition-all tracking-wide text-white min-h-[140px]",
                                        value: "{cover_letter}",
                                        oninput: move |e| cover_letter.set(e.value())
                                    }
                                }
                            }
                         }

                        if !error_msg().is_empty() {
                             div {
                                class: "text-red-500 bg-red-500/10 border border-red-500/20 text-[10px] font-black uppercase tracking-widest text-center py-3 rounded",
                                "{error_msg}"
                            }
                        }

                        if uploading() {
                            div { class: "flex items-center justify-center gap-3 py-4",
                                div { class: "w-4 h-4 border-t-2 border-b-2 border-accent-color rounded-full animate-spin" }
                                span { class: "text-[10px] font-black uppercase tracking-widest opacity-60", "Uploading assets..." }
                            }
                        }

                        div {
                            button {
                                onclick: create_app,
                                disabled: "{uploading}",
                                class: "noir-btn w-full py-4 text-xs font-black uppercase tracking-[0.4em] active:scale-95 disabled:opacity-30 disabled:cursor-not-allowed transition-all",
                                "Create Application"
                            }
                        }
                    }
                }
            }
    }
}
