use dioxus::prelude::*;
use reqwest::Client;
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

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut error_msg = use_signal(|| "".to_string());
    let nav = use_navigator();

    let on_submit = move |evt: FormEvent| async move {
        evt.prevent_default();
        let client = Client::new();
        let payload = LoginPayload {
            email: email(),
            password: password(),
        };

        match client
            .post(format!(
                "{}/auth/login",
                crate::services::application_service::API_BASE_URL
            ))
            .json(&payload)
            .send()
            .await
        {
            Ok(resp) => {
                if resp.status().is_success() {
                    let body = resp.json::<LoginResponse>().await;
                    match body {
                        Ok(data) => {
                            // TODO: Store token in storage or context
                            dioxus_logger::tracing::info!("Login success! Token: {}", data.token);
                            // nav.push(Route::Home {}); // Using string path for now
                            nav.push("/");
                        }
                        Err(_) => error_msg.set("Failed to parse response".to_string()),
                    }
                } else {
                    error_msg.set("Invalid credentials".to_string());
                }
            }
            Err(e) => {
                error_msg.set(format!("Request failed: {}", e));
            }
        }
    };

    rsx! {
        div {
            class: "max-w-md mx-auto mt-10 p-6 bg-white rounded-lg shadow-md",
            h1 { class: "text-2xl font-bold mb-6 text-center", "Admin Login" }

            if !error_msg().is_empty() {
                div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4",
                    "{error_msg}"
                }
            }

            form {
                onsubmit: on_submit,
                div { class: "mb-4",
                    label { class: "block text-gray-700 text-sm font-bold mb-2", "Email" }
                    input {
                        class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                        r#type: "email",
                        value: "{email}",
                        oninput: move |e| email.set(e.value())
                    }
                }
                div { class: "mb-6",
                    label { class: "block text-gray-700 text-sm font-bold mb-2", "Password" }
                    input {
                        class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                        r#type: "password",
                        value: "{password}",
                        oninput: move |e| password.set(e.value())
                    }
                }
                div { class: "flex items-center justify-between",
                    button {
                        class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline",
                        r#type: "submit",
                        "Sign In"
                    }
                }
            }
        }
    }
}
