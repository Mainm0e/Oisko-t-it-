use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

#[component]
pub fn AdminLogin() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut error_msg = use_signal(|| "".to_string());
    let navigator = use_navigator();

    let onsubmit = move |evt: FormEvent| async move {
        evt.prevent_default();
        // Prevent default submission handled by Dioxus usually, but we want manual control

        let client = reqwest::Client::new();
        let payload = LoginPayload {
            email: email(),
            password: password(),
        };

        // TODO: Make base URL configurable
        let res = client
            .post("http://localhost:3000/api/auth/login")
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    if let Ok(data) = response.json::<LoginResponse>().await {
                        // TODO: Store token securely (e.g. specialized storage crate or JS interop)
                        // For now just console log or basic storage if possible
                        // Dioxus web doesn't have direct access to localStorage without web-sys or eval

                        let token = data.token;
                        // Using eval to set localStorage for now as it's the quick way in Dioxus
                        let _ = document::eval(&format!(
                            "localStorage.setItem('admin_token', '{}')",
                            token
                        ));

                        navigator.push("/admin/dashboard"); // Assuming we will have a dashboard
                    }
                } else {
                    if let Ok(err) = response.json::<ErrorResponse>().await {
                        error_msg.set(err.error);
                    } else {
                        error_msg.set("Login failed".to_string());
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
                    "Sign in to admin account"
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
                        div {
                            class: "flex items-center justify-between",
                            label {
                                r#for: "password",
                                class: "block text-sm font-medium leading-6 text-gray-300",
                                "Password"
                            }
                        }
                        div {
                            class: "mt-2",
                            input {
                                id: "password",
                                name: "password",
                                r#type: "password",
                                autocomplete: "current-password",
                                required: true,
                                class: "block w-full rounded-md border-0 py-1.5 bg-gray-900/50 text-white shadow-sm ring-1 ring-inset ring-white/10 placeholder:text-gray-500 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6",
                                value: "{password}",
                                oninput: move |e| password.set(e.value())
                            }
                        }
                    }

                    if !error_msg().is_empty() {
                        div {
                            class: "text-red-400 text-sm text-center bg-red-900/20 py-2 rounded border border-red-500/20",
                            "{error_msg}"
                        }
                    }

                    div {
                        button {
                            r#type: "submit",
                            class: "flex w-full justify-center rounded-md bg-indigo-500 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500 transition-colors duration-200",
                            "Sign in"
                        }
                    }

                    div {
                        class: "text-center text-sm",
                        Link {
                            to: "/admin/register",
                            class: "font-semibold text-indigo-400 hover:text-indigo-300",
                            "Don't have an account? Register"
                        }
                    }
                }
            }
        }
    }
}
