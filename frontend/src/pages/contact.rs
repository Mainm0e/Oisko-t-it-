use dioxus::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct ContactPayload {
    name: String,
    email: String,
    link: Option<String>,
    message: String,
}

#[component]
pub fn Contact() -> Element {
    let mut name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut link = use_signal(|| String::new());
    let mut message = use_signal(|| String::new());
    let mut is_submitting = use_signal(|| false);
    let mut success_message = use_signal(|| Option::<String>::None);
    let mut error_message = use_signal(|| Option::<String>::None);

    let submit = move |evt: Event<MouseData>| async move {
        evt.stop_propagation();
        if is_submitting() {
            return;
        }

        is_submitting.set(true);
        success_message.set(None);
        error_message.set(None);

        let payload = ContactPayload {
            name: name(),
            email: email(),
            link: if link().is_empty() {
                None
            } else {
                Some(link())
            },
            message: message(),
        };

        let client = reqwest::Client::new();
        // Use the shared API_BASE_URL which defaults to localhost:3000 but can be overridden via API_URL env var
        use crate::services::application_service::API_BASE_URL;
        let url = format!("{}/contact", API_BASE_URL);

        match client.post(url).json(&payload).send().await {
            Ok(res) => {
                if res.status().is_success() {
                    success_message.set(Some(
                        "Transmission successful. Stand by for response.".to_string(),
                    ));
                    name.set(String::new());
                    email.set(String::new());
                    link.set(String::new());
                    message.set(String::new());
                } else {
                    error_message.set(Some("Transmission failed. Signal lost.".to_string()));
                }
            }
            Err(_) => {
                error_message.set(Some("Network error. Comm link offline.".to_string()));
            }
        }
        is_submitting.set(false);
    };

    rsx! {
        div { class: "max-w-3xl mx-auto px-4 py-20 min-h-screen",
            div { class: "mb-12 text-center",
                h1 { class: "text-4xl md:text-6xl font-black uppercase tracking-tighter mb-4 glitch-text",
                    style: "text-shadow: 0 0 20px var(--accent-glow); color: var(--text-color);",
                    "Value Detected?"
                }
                p { class: "font-mono text-sm opacity-60 tracking-[0.2em] uppercase",
                    "If you see potential in my signals, transmit your job offer coordinates."
                }
            }

            div { class: "glass p-8 md:p-12 rounded-lg border border-white/5 relative overflow-hidden",
                // Decorative elements
                div { class: "absolute top-0 right-0 w-32 h-32 bg-accent-glow blur-[80px] opacity-20 -z-10" }

                if let Some(msg) = success_message() {
                    div { class: "mb-8 p-4 border border-green-500/30 bg-green-500/10 rounded flex items-center gap-4 animate-in fade-in slide-in-from-top-2",
                        div { class: "w-2 h-2 rounded-full bg-green-500 animate-pulse" }
                        p { class: "font-mono text-sm text-green-400 uppercase tracking-widest", "{msg}" }
                    }
                }

                if let Some(err) = error_message() {
                    div { class: "mb-8 p-4 border border-red-500/30 bg-red-500/10 rounded flex items-center gap-4 animate-in fade-in slide-in-from-top-2",
                        div { class: "w-2 h-2 rounded-full bg-red-500 animate-pulse" }
                        p { class: "font-mono text-sm text-red-400 uppercase tracking-widest", "{err}" }
                    }
                }

                form { class: "space-y-8", onsubmit: move |e| e.prevent_default(),
                    div { class: "space-y-2",
                        label { class: "block text-xs font-bold uppercase tracking-[0.2em] text-gray-400", "Recruiter Identity // Name" }
                        input {
                            class: "w-full bg-black/80 border border-white/20 rounded p-4 text-sm font-mono focus:border-accent-color focus:outline-none transition-all placeholder:text-gray-600",
                            placeholder: "ENTER_DESIGNATION",
                            value: "{name}",
                            oninput: move |e| name.set(e.value()),
                            disabled: is_submitting()
                        }
                    }

                    div { class: "space-y-2",
                        label { class: "block text-xs font-bold uppercase tracking-[0.2em] text-gray-400", "Secure Frequency // Email" }
                        input {
                            class: "w-full bg-black/80 border border-white/20 rounded p-4 text-sm font-mono focus:border-accent-color focus:outline-none transition-all placeholder:text-gray-600",
                            type: "email",
                            placeholder: "ENTER_COMM_ADDRESS",
                            value: "{email}",
                            oninput: move |e| email.set(e.value()),
                            disabled: is_submitting()
                        }
                    }

                    div { class: "space-y-2",
                        label { class: "block text-xs font-bold uppercase tracking-[0.2em] text-gray-400", "Mission Coordinates // Job Link" }
                        input {
                            class: "w-full bg-black/80 border border-white/20 rounded p-4 text-sm font-mono focus:border-accent-color focus:outline-none transition-all placeholder:text-gray-600",
                            type: "url",
                            placeholder: "ENTER_OFFER_URL",
                            value: "{link}",
                            oninput: move |e| link.set(e.value()),
                            disabled: is_submitting()
                        }
                    }

                    div { class: "space-y-2",
                        label { class: "block text-xs font-bold uppercase tracking-[0.2em] text-gray-400", "Offer Details // Message" }
                        textarea {
                            class: "w-full bg-black/80 border border-white/20 rounded p-4 text-sm font-mono focus:border-accent-color focus:outline-none transition-all h-40 resize-none placeholder:text-gray-600",
                            placeholder: "ENTER_OPPORTUNITY_DATA...",
                            value: "{message}",
                            oninput: move |e| message.set(e.value()),
                            disabled: is_submitting()
                        }
                    }

                    button {
                        class: "w-full py-4 bg-white text-black font-black uppercase tracking-[0.4em] text-sm hover:scale-[1.02] hover:shadow-[0_0_30px_rgba(255,255,255,0.3)] active:scale-[0.99] transition-all disabled:opacity-50 disabled:cursor-not-allowed relative overflow-hidden group",
                        onclick: submit,
                        disabled: is_submitting(),

                        if is_submitting() {
                             span { "UPLOADING_OFFER..." }
                        } else {
                             span { "TRANSMIT_OFFER" }
                        }

                        // Button hover effect
                        div { class: "absolute inset-0 bg-white/20 translate-x-[-100%] group-hover:translate-x-[100%] transition-transform duration-500" }
                    }
                }
                div { class: "mt-8 text-center",
                    Link {
                        to: crate::Route::Home {},
                        class: "text-xs font-mono uppercase tracking-[0.2em] opacity-40 hover:opacity-100 transition-opacity hover:text-accent-color",
                        "<< RETURN_TO_COMMAND_CENTER"
                    }
                }
            }
        }
    }
}
