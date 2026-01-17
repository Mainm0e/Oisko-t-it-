use dioxus::prelude::*;
use std::time::Duration;

#[component]
pub fn DataCounter(
    value: i32,
    duration_ms: Option<u64>,
    prefix: Option<String>,
    suffix: Option<String>,
) -> Element {
    let mut current = use_signal(|| 0);
    let duration = duration_ms.unwrap_or(1500);

    use_effect(move || {
        let target = value;
        spawn(async move {
            let steps = 30;
            let step_time = duration / steps;
            let target_f = target as f32;
            let start_f = current() as f32;

            if target == current() {
                return;
            }

            let increment = (target_f - start_f) / steps as f32;

            for _ in 0..steps {
                let next = current() + increment.ceil() as i32;
                if (increment > 0.0 && next >= target) || (increment < 0.0 && next <= target) {
                    break;
                }
                current.set(next);
                gloo_timers::future::sleep(Duration::from_millis(step_time)).await;
            }
            current.set(target);
        });
    });

    rsx! {
        span {
            if let Some(p) = prefix { "{p}" }
            "{current}"
            if let Some(s) = suffix { "{s}" }
        }
    }
}
