use crate::models::application::{Comment, CreateComment, PublicApplicationDetail};
use crate::services::application_service::{
    create_comment, get_comments, get_public_application_detail, BASE_URL,
};
use dioxus::prelude::*;

const DATE_FMT: &str = "%Y-%m-%d %H:%M";

#[component]
pub fn ApplicationDetail(id: String) -> Element {
    let application_resource = use_resource(use_reactive(&id, |id| async move {
        get_public_application_detail(&id).await
    }));

    let mut comments_resource =
        use_resource(use_reactive(
            &id,
            |id| async move { get_comments(&id).await },
        ));

    let mut new_comment_content = use_signal(|| "".to_string());
    let mut visitor_name = use_signal(|| "".to_string());
    let mut submitting = use_signal(|| false);

    rsx! {
        div { class: "max-w-4xl mx-auto px-4 py-20 min-h-screen",
            // Back Button
            Link {
                to: crate::Route::Home {},
                class: "noir-btn inline-flex items-center gap-2 mb-8 text-sm",
                "← BACK TO MISSION LOG"
            }

            match &*application_resource.read() {
                Some(Ok(app)) => rsx! {
                    div { class: "noir-card p-8 md:p-12 mb-12 bg-[var(--card-bg)]",
                        // Header
                        div { class: "border-b border-[var(--border-color)] pb-8 mb-8",
                            div { class: "flex justify-between items-start",
                                div {
                                    h1 { class: "text-4xl md:text-5xl font-bold mb-2", style: "color: var(--text-color);", "{app.company}" }
                                    if let Some(website) = &app.company_website {
                                        a {
                                            href: "{website}",
                                            target: "_blank",
                                            class: "text-accent text-sm tracking-widest hover:underline",
                                            style: "color: var(--text-color); opacity: 0.7;",
                                            "OFFICIAL FREQUENCY_↗"
                                        }
                                    }
                                }
                                span {
                                    class: "px-3 py-1 text-xs font-bold uppercase tracking-widest border",
                                    style: "color: var(--text-color); border-color: var(--text-color);",
                                    "{app.status}"
                                }
                            }
                        }

                        // Grid Details
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-8 mb-8",
                            div {
                                h3 { class: "text-xs uppercase tracking-widest mb-2 opacity-50", "Role" }
                                p { class: "text-xl font-medium", "{app.role}" }
                            }
                             div {
                                h3 { class: "text-xs uppercase tracking-widest mb-2 opacity-50", "Salary Data" }
                                p { class: "text-xl font-mono", "{app.salary.clone().unwrap_or_else(|| \"CONFIDENTIAL\".to_string())}" }
                            }
                            div {
                                h3 { class: "text-xs uppercase tracking-widest mb-2 opacity-50", "Date Logged" }
                                p { class: "font-mono", "{app.created_at.format(DATE_FMT)}" }
                            }
                        }

                        // Cover Letter Section
                        if let Some(letter) = &app.cover_letter {
                            div { class: "mb-8",
                                h3 { class: "text-xs uppercase tracking-widest mb-4 opacity-50", "Transmission Intercept (Cover Letter)" }
                                div { class: "p-6 border border-[var(--border-color)] bg-[var(--bg-color)] font-mono text-sm leading-relaxed whitespace-pre-wrap opacity-80",
                                    "{letter}"
                                }
                            }
                        }

                        // Action Buttons
                        if let Some(cv_path) = &app.cv_path {
                             a {
                                href: "{BASE_URL}{cv_path}",
                                target: "_blank",
                                class: "noir-btn w-full block text-center py-4 hover:bg-[var(--text-color)] hover:text-[var(--bg-color)] transition-colors",
                                "DOWNLOAD TACTICAL DATA (CV)"
                            }
                        }
                    }

                    // Comments Section
                    div { class: "mt-16",
                        h3 { class: "text-2xl font-bold mb-8 tracking-widest", "COMMUNICATIONS CHANNEL" }

                        // Add Comment Form
                        div { class: "noir-card p-6 mb-12",
                            h4 { class: "text-sm uppercase tracking-widest mb-4", "Establish Link" }
                             div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-4",
                                input {
                                    class: "bg-transparent border border-[var(--border-color)] p-3 focus:outline-none focus:border-[var(--text-color)] transition-colors",
                                    placeholder: "CODENAME (Optional)",
                                    value: "{visitor_name}",
                                    oninput: move |e| visitor_name.set(e.value())
                                }
                            }
                            textarea {
                                class: "w-full bg-transparent border border-[var(--border-color)] p-3 focus:outline-none focus:border-[var(--text-color)] h-32 mb-4 transition-colors",
                                placeholder: "TRANSMIT MESSAGE...",
                                value: "{new_comment_content}",
                                oninput: move |e| new_comment_content.set(e.value())
                            }
                            button {
                                class: "noir-btn w-full md:w-auto",
                                disabled: "{submitting}",
                                onclick: {
                                    let app_id = app.id.to_string();
                                    move |_| {
                                        submitting.set(true);
                                        let content = new_comment_content();
                                        let name = if visitor_name().trim().is_empty() { "Anonymous".to_string() } else { visitor_name() };
                                        let app_id = app_id.clone();

                                        spawn(async move {
                                            if !content.trim().is_empty() {
                                                 let _ = create_comment(&app_id, CreateComment {
                                                    visitor_name: name,
                                                    content: content,
                                                }).await;
                                                new_comment_content.set("".to_string());
                                                comments_resource.restart();
                                            }
                                            submitting.set(false);
                                        });
                                    }
                                },
                                if submitting() { "TRANSMITTING..." } else { "SEND MESSAGE" }
                            }
                        }

                        // Comments List
                        match &*comments_resource.read() {
                            Some(Ok(comments)) => rsx! {
                                div { class: "space-y-6",
                                    for comment in comments {
                                        div { class: "border-l-2 border-[var(--border-color)] pl-6 py-2",
                                            div { class: "flex justify-between items-baseline mb-2",
                                                span { class: "font-bold tracking-wide", "{comment.visitor_name}" }
                                                span { class: "text-xs font-mono opacity-50", "{comment.created_at.format(DATE_FMT)}" }
                                            }
                                            p { class: "opacity-80 leading-relaxed", "{comment.content}" }
                                        }
                                    }
                                    if comments.is_empty() {
                                        p { class: "text-center opacity-50 font-mono py-8", "NO SIGNALS DETECTED." }
                                    }
                                }
                            },
                            _ => rsx! { div { "Loading comms..." } }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { class: "text-red-500 font-mono text-center py-20", "ACCESS DENIED: {e}" }
                },
                None => rsx! {
                    div { class: "flex justify-center py-20",
                        div { class: "animate-spin rounded-full h-8 w-8 border-t-2 border-b-2", style: "border-color: var(--text-color);" }
                    }
                }
            }
        }
    }
}
