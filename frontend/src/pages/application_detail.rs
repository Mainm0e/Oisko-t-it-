use crate::models::application::CreateComment;
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
        div { class: "max-w-7xl mx-auto px-4 py-20 min-h-screen scanline",
            // Back Button & Navigation context
            div { class: "flex justify-between items-center mb-12",
                Link {
                    to: crate::Route::Home {},
                    class: "noir-btn flex items-center gap-2 text-xs",
                    "← RTN_TO_LOG"
                }
                div { class: "flex items-center gap-2 text-[10px] tracking-widest font-mono opacity-40",
                    "DOSSIER_ID:"
                    span { class: "text-accent-color", "{id}" }
                }
            }

            match &*application_resource.read() {
                Some(Ok(app)) => {
                    let date_str = app.created_at.format(DATE_FMT).to_string();
                    let page_title = format!("{} | Dossier", app.company);
                    rsx! {
                        document::Title { "{page_title}" }
                        div { class: "grid grid-cols-1 lg:grid-cols-3 gap-12",
                            // Left Column - Core Intel
                            div { class: "lg:col-span-2 space-y-12",
                                // Primary Briefing
                                div { class: "glass p-10 rounded-sm relative overflow-hidden",
                                    div { class: "absolute top-0 right-0 w-32 h-32 bg-accent-glow blur-[100px] opacity-10 pointer-events-none" }

                                    div { class: "flex flex-col md:flex-row justify-between items-start gap-8 mb-12",
                                        div { class: "flex items-start gap-6",
                                            if let Some(logo) = &app.logo_url {
                                                img {
                                                    src: "{logo}",
                                                    class: "w-24 h-24 rounded-lg bg-white/5 object-contain border border-white/10 p-2 shadow-2xl",
                                                }
                                            }
                                            div {
                                                h1 { class: "text-5xl md:text-7xl font-black mb-2 tracking-tighter", "{app.company}" }
                                                if let Some(website) = &app.company_website {
                                                    a {
                                                        href: "{website}",
                                                        target: "_blank",
                                                        class: "text-xs tracking-[0.3em] font-bold hover:text-accent-color transition-colors flex items-center gap-2 opacity-50",
                                                        "OFFICIAL_SRC_↗"
                                                    }
                                                }
                                            }
                                        }
                                        div {
                                            class: "px-6 py-2 text-xs font-black uppercase tracking-[0.4em] border skew-x-[-20deg]",
                                            style: match app.status.as_str() {
                                                "Offer" => "background: var(--status-offer); color: black; border-color: var(--status-offer); box-shadow: 0 0 30px rgba(16, 185, 129, 0.4);",
                                                "Rejected" => "background: transparent; color: var(--status-rejected); border-color: var(--status-rejected); opacity: 0.6;",
                                                "Interviewing" => "background: var(--status-interview); color: black; border-color: var(--status-interview); box-shadow: 0 0 30px rgba(14, 165, 233, 0.4);",
                                                _ => "background: rgba(255,255,255,0.05); color: white; border-color: white/20;",
                                            },
                                            div { class: "skew-x-[20deg]", "{app.status}" }
                                        }
                                    }

                                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-10 py-10 border-y border-white/5",
                                        div {
                                            div { class: "text-[10px] uppercase tracking-[0.3em] opacity-40 mb-2", "Designation" }
                                            div { class: "text-2xl font-bold", "{app.role}" }
                                        }
                                        div {
                                            div { class: "text-[10px] uppercase tracking-[0.3em] opacity-40 mb-2", "Comp_Data" }
                                            div { class: "text-2xl font-mono text-accent-color", "{app.salary.clone().unwrap_or_else(|| \"--- REDACTED ---\".to_string())}" }
                                        }
                                        div {
                                            div { class: "text-[10px] uppercase tracking-[0.3em] opacity-40 mb-2", "Init_Log" }
                                            div { class: "text-xl font-mono opacity-80", "{date_str}" }
                                        }
                                    }

                                    if let Some(desc) = &app.description {
                                        div { class: "mt-12 space-y-4",
                                            h3 { class: "text-[10px] uppercase tracking-[0.5em] font-black opacity-30", "INTEL_SUMMARY_PT_01" }
                                            p { class: "text-lg leading-relaxed opacity-80 font-light", "{desc}" }
                                        }
                                    }
                                }

                                // Transmission Intercept
                                if let Some(letter) = &app.cover_letter {
                                    div { class: "glass p-10 rounded-sm relative overflow-hidden",
                                        h3 { class: "text-[10px] uppercase tracking-[0.5em] font-black opacity-30 mb-8", "TRANS_INTERCEPT_PT_02" }
                                        div { class: "bg-black/40 p-8 rounded border border-white/5 font-mono text-sm leading-relaxed whitespace-pre-wrap opacity-90 relative",
                                            div { class: "absolute top-4 right-4 text-[8px] opacity-20", "DECRYPT_LVL_9" }
                                            "{letter}"
                                        }
                                    }
                                }
                            }

                            // Right Column - Comms & Actions
                            div { class: "space-y-12",
                                // Tactical Data Links
                                if let Some(cv_path) = &app.cv_path {
                                    div { class: "glass p-8 rounded-sm",
                                        h3 { class: "text-[10px] uppercase tracking-[0.5em] font-black opacity-30 mb-6", "RESOURCES" }
                                        a {
                                            href: "{BASE_URL}{cv_path}",
                                            target: "_blank",
                                            class: "noir-btn w-full block text-center py-6 animate-glow",
                                            "DWNLD_TACTICAL_CV"
                                        }
                                    }
                                }

                                // Comms Channel
                                div { class: "glass p-8 rounded-sm space-y-8",
                                    h3 { class: "text-[10px] uppercase tracking-[0.5em] font-black opacity-30", "COMM_CHANNEL" }

                                    // New Message
                                    div { class: "space-y-4",
                                        input {
                                            class: "w-full bg-white/5 border border-white/10 p-3 text-sm focus:outline-none focus:border-accent-color transition-colors font-mono",
                                            placeholder: "ID_TAG (OPTIONAL)",
                                            value: "{visitor_name}",
                                            oninput: move |e| visitor_name.set(e.value())
                                        }
                                        textarea {
                                            class: "w-full bg-white/5 border border-white/10 p-3 text-sm focus:outline-none focus:border-accent-color h-32 transition-colors font-mono",
                                            placeholder: "TRANSMIT_SIG...",
                                            value: "{new_comment_content}",
                                            oninput: move |e| new_comment_content.set(e.value())
                                        }
                                        button {
                                            class: "noir-btn w-full py-4 text-xs tracking-widest",
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
                                            if submitting() { "TRANSMITTING..." } else { "EXEC_SEND" }
                                        }
                                    }

                                    // Message Log
                                    match &*comments_resource.read() {
                                        Some(Ok(comments)) => rsx! {
                                            div { class: "space-y-4 max-h-[400px] overflow-y-auto pr-2 custom-scrollbar",
                                                for comment in comments {
                                                    {
                                                        let time_str = comment.created_at.format("%H:%M").to_string();
                                                        rsx! {
                                                            div { class: "p-4 border-l-2 border-accent-color/30 bg-white/5",
                                                                div { class: "flex justify-between items-center mb-1",
                                                                    span { class: "text-[10px] font-black tracking-widest text-accent-color", "{comment.visitor_name.to_uppercase()}" }
                                                                    span { class: "text-[8px] font-mono opacity-30", "{time_str}" }
                                                                }
                                                                p { class: "text-sm opacity-80 leading-relaxed", "{comment.content}" }
                                                            }
                                                        }
                                                    }
                                                }
                                                if comments.is_empty() {
                                                    p { class: "text-center opacity-20 font-mono text-[10px] py-10", "--- NO_SIGNALS_RECOGNIZED ---" }
                                                }
                                            }
                                        },
                                        _ => rsx! { div { class: "animate-pulse py-10 text-center text-[10px] opacity-30 tracking-[0.5em]", "SCANNING_CHANNELS..." } }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { class: "glass p-20 text-center border-red-500/20",
                        h2 { class: "text-red-500 mb-4", "ACCESS_DENIED_0x00FF" }
                        p { class: "font-mono text-sm opacity-50", "{e}" }
                    }
                },
                None => rsx! {
                    div { class: "flex flex-col items-center justify-center py-40 gap-6",
                        div { class: "animate-spin rounded-full h-16 w-16 border-t-2 border-accent-color shadow-[0_0_20px_var(--accent-glow)]" }
                        p { class: "text-[10px] uppercase tracking-[0.5em] animate-pulse opacity-40", "RETRIEVING_DATA_CORES..." }
                    }
                }
            }
        }
    }
}
