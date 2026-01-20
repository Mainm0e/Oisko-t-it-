use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct VerifyPayload {
    token: String,
}

#[derive(Deserialize)]
struct VerifyResponse {
    message: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

#[component]
pub fn VerifyEmail() -> Element {
    let mut token_input = use_signal(|| "".to_string());
    let mut message = use_signal(|| "".to_string());
    let mut is_error = use_signal(|| false);
    // let navigator = use_navigator();

    // Check for query param 'token' on mount
    // Dioxus router doesn't have easy query param extraction hooks in 0.5/0.6 yet without some manual parsing
    // or using url crate. For now, we'll just provide an input box.
    // Ideally: extract ?token=... from window location or router logic.

    let verify = move |_| async move {
        let client = reqwest::Client::new();
        let payload = VerifyPayload {
            token: token_input(),
        };

        match client
            .post(format!(
                "{}/auth/verify",
                crate::services::application_service::API_BASE_URL
            ))
            .json(&payload)
            .send()
            .await
        {
            Ok(resp) => {
                if resp.status().is_success() {
                    message.set("Verification success! Redirecting to login...".to_string());
                    is_error.set(false);
                    // Wait 2s then redirect
                    let _ = document::eval(
                        "setTimeout(() => { window.location.href = '/admin/login' }, 2000)",
                    )
                    .await;
                } else {
                    is_error.set(true);
                    if let Ok(err) = resp.json::<ErrorResponse>().await {
                        message.set(err.error);
                    } else {
                        message.set("Verification failed".to_string());
                    }
                }
            }
            Err(e) => {
                is_error.set(true);
                message.set(format!("Network error: {}", e));
            }
        }
    };

    let msg_color_class = if is_error() {
        "text-red-400 bg-red-900/20 border-red-500/20"
    } else {
        "text-green-400 bg-green-900/20 border-green-500/20"
    };

    rsx! {
        div {
            class: "flex min-h-full flex-col justify-center px-6 py-12 lg:px-8 bg-[#0f1116] text-white",
            div {
                class: "sm:mx-auto sm:w-full sm:max-w-sm",
                h2 {
                    class: "mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-white",
                    "Verify your account"
                }
                p {
                    class: "mt-2 text-center text-sm text-gray-400",
                    "Check your server logs for the token."
                }
            }

            div {
                class: "mt-10 sm:mx-auto sm:w-full sm:max-w-sm",
                div {
                    class: "space-y-6 bg-white/5 p-8 rounded-lg border border-white/10 backdrop-blur-sm shadow-xl",

                    div {
                        label {
                            class: "block text-sm font-medium leading-6 text-gray-300",
                            "Verification Token"
                        }
                        div {
                            class: "mt-2",
                            input {
                                r#type: "text",
                                class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6",
                                value: "{token_input}",
                                oninput: move |e| token_input.set(e.value())
                            }
                        }
                    }

                    if !message().is_empty() {
                         div {
                            class: "{msg_color_class} text-sm text-center py-2 rounded border",
                            "{message}"
                        }
                    }

                    div {
                        button {
                            onclick: verify,
                            class: "flex w-full justify-center rounded-md bg-indigo-500 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-400",
                            "Verify"
                        }
                    }
                }
            }
        }
    }
}
