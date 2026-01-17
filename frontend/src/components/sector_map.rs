use dioxus::prelude::*;

#[component]
pub fn SectorMap(class: Option<String>) -> Element {
    rsx! {
        div {
            class: format!("relative aspect-square overflow-hidden rounded-full border border-white/5 bg-black/20 glass {}", class.unwrap_or_default()),

            // Rotating Radar Line
            div {
                class: "absolute inset-0 pointer-events-none",
                style: "background: conic-gradient(from 0deg, var(--accent-color) 0deg, transparent 90deg); opacity: 0.1; animation: radar-rotate 4s linear infinite;"
            }

            // Grid Lines
            div { class: "absolute inset-0 flex items-center justify-center pointer-events-none",
                div { class: "w-full h-[1px] bg-white/5" }
                div { class: "absolute w-[1px] h-full bg-white/5" }
                div { class: "absolute w-3/4 h-3/4 border border-white/5 rounded-full" }
                div { class: "absolute w-1/2 h-1/2 border border-white/5 rounded-full" }
                div { class: "absolute w-1/4 h-1/4 border border-white/5 rounded-full" }
            }

            // Pulsing "Targets" (Applications)
            TargetPoint { top: "25%", left: "30%", color: "var(--status-offer)", label: "OP_SUCCESS" }
            TargetPoint { top: "60%", left: "70%", color: "var(--status-interview)", label: "INTV_SCAN" }
            TargetPoint { top: "40%", left: "80%", color: "var(--accent-color)", label: "APP_DISPATCH" }
            TargetPoint { top: "75%", left: "20%", color: "var(--status-rejected)", label: "SIG_LOST" }
        }
    }
}

#[component]
fn TargetPoint(top: String, left: String, color: String, label: String) -> Element {
    rsx! {
        div {
            class: "absolute group cursor-help",
            style: "top: {top}; left: {left};",

            // Pulse
            div {
                class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-4 h-4 rounded-full",
                style: "background: {color}; animation: point-pulse 2s infinite ease-out; opacity: 0.5;"
            }

            // Core
            div {
                class: "w-1.5 h-1.5 rounded-full relative",
                style: "background: {color}; box-shadow: 0 0 10px {color};"
            }

            // Label
            span {
                class: "absolute left-4 top-0 whitespace-nowrap text-[8px] font-black tracking-widest opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none",
                style: "color: {color};",
                "{label}"
            }
        }
    }
}
