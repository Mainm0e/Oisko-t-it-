use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RegisterPayload {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

#[component]
pub fn AdminRegister() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut confirm_password = use_signal(|| "".to_string());
    let mut error_msg = use_signal(|| "".to_string());
    let mut success_msg = use_signal(|| "".to_string());
    // let navigator = use_navigator(); // Redirect could vary based on user preference

    let onsubmit = move |evt: FormEvent| async move {
        evt.prevent_default();
        error_msg.set("".to_string());
        success_msg.set("".to_string());

        if password() != confirm_password() {
            error_msg.set("Passwords do not match".to_string());
            return;
        }

        let client = reqwest::Client::new();
        let payload = RegisterPayload {
            email: email(),
            password: password(),
        };

        // TODO: Make base URL configurable
        let res = client
            .post("http://localhost:3000/api/auth/register")
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    success_msg.set("Registration successful! Please check your email/logs for verification token.".to_string());
                    // Optionally clear form
                    email.set("".to_string());
                    password.set("".to_string());
                    confirm_password.set("".to_string());
                } else {
                    if let Ok(err) = response.json::<ErrorResponse>().await {
                        error_msg.set(err.error);
                    } else {
                        error_msg.set("Registration failed".to_string());
                    }
                }
            }
            Err(e) => {
                error_msg.set(format!("Network error: {}", e));
            }
        }
    };

    rsx! {
        div {
            class: "flex min-h-full flex-col justify-center px-6 py-12 lg:px-8 bg-[#0f1116] text-white",
            div {
                class: "sm:mx-auto sm:w-full sm:max-w-sm",
                h2 {
                    class: "mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-white",
                    "Create admin account"
                }
            }

            div {
                class: "mt-10 sm:mx-auto sm:w-full sm:max-w-sm",
                form {
                    class: "space-y-6 bg-white/5 p-8 rounded-lg border border-white/10 backdrop-blur-sm shadow-xl",
                    onsubmit: onsubmit,

                    div {
                        label {
                            r#for: "email",
                            class: "block text-sm font-medium leading-6 text-gray-300",
                            "Email address"
                        }
                        div {
                            class: "mt-2",
                            input {
                                id: "email",
                                name: "email",
                                r#type: "email",
                                autocomplete: "email",
                                required: true,
                                class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6",
                                value: "{email}",
                                oninput: move |e| email.set(e.value())
                            }
                        }
                    }

                    div {
                        label {
                            r#for: "password",
                            class: "block text-sm font-medium leading-6 text-gray-300",
                            "Password"
                        }
                        div {
                            class: "mt-2",
                            input {
                                id: "password",
                                name: "password",
                                r#type: "password",
                                autocomplete: "new-password",
                                required: true,
                                class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6",
                                value: "{password}",
                                oninput: move |e| password.set(e.value())
                            }
                        }
                    }

                    div {
                        label {
                            r#for: "confirm_password",
                            class: "block text-sm font-medium leading-6 text-gray-300",
                            "Confirm Password"
                        }
                        div {
                            class: "mt-2",
                            input {
                                id: "confirm_password",
                                name: "confirm_password",
                                r#type: "password",
                                autocomplete: "new-password",
                                required: true,
                                class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6",
                                value: "{confirm_password}",
                                oninput: move |e| confirm_password.set(e.value())
                            }
                        }
                    }

                    if !error_msg().is_empty() {
                        div {
                            class: "text-red-400 text-sm text-center bg-red-900/20 py-2 rounded border border-red-500/20",
                            "{error_msg}"
                        }
                    }

                    if !success_msg().is_empty() {
                         div {
                            class: "text-emerald-400 text-sm text-center bg-emerald-900/20 py-2 rounded border border-emerald-500/20",
                            "{success_msg}"
                        }
                    }

                    div {
                        button {
                            r#type: "submit",
                            class: "flex w-full justify-center rounded-md bg-indigo-500 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500 transition-colors duration-200",
                            "Register"
                        }
                    }

                    div {
                        class: "text-center text-sm",
                        Link {
                            to: "/admin/login",
                            class: "font-semibold text-indigo-400 hover:text-indigo-300",
                            "Already have an account? Sign in"
                        }
                    }
                }
            }
        }
    }
}
