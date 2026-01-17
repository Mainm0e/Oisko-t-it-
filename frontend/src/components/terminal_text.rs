use dioxus::prelude::*;
use rand::Rng;
use std::time::Duration;

#[component]
pub fn TerminalText(
    text: String,
    delay: Option<u64>,
    speed: Option<u64>,
    cursor: Option<bool>,
    decode: Option<bool>,
    class: Option<String>,
) -> Element {
    let mut revealed_text = use_signal(|| String::new());
    let mut is_complete = use_signal(|| false);
    let target_text = text.clone();
    let speed_val = speed.unwrap_or(50);
    let delay_val = delay.unwrap_or(0);
    let show_cursor = cursor.unwrap_or(true);
    let use_decode = decode.unwrap_or(false);

    use_effect(move || {
        let target_text = target_text.clone();
        spawn(async move {
            if delay_val > 0 {
                gloo_timers::future::sleep(Duration::from_millis(delay_val)).await;
            }

            let chars: Vec<char> = target_text.chars().collect();
            let mut current = String::new();

            for (_i, &c) in chars.iter().enumerate() {
                if use_decode {
                    // Quick random character decoding effect
                    for _ in 0..2 {
                        let mut rng = rand::rng();
                        let random_char: char = rng.random_range(33..126) as u8 as char;
                        revealed_text.set(format!("{}{}", current, random_char));
                        gloo_timers::future::sleep(Duration::from_millis(speed_val / 3)).await;
                    }
                }

                current.push(c);
                revealed_text.set(current.clone());
                gloo_timers::future::sleep(Duration::from_millis(speed_val)).await;
            }
            is_complete.set(true);
        });
    });

    rsx! {
        span {
            class: class.unwrap_or_default(),
            "data-text": "{text}",
            "{revealed_text}"
            if show_cursor && !is_complete() {
                span { class: "animate-pulse ml-1", "_" }
            }
        }
    }
}
