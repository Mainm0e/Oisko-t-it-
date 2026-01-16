use crate::models::application::{DailyCount, StatusCount};
use dioxus::prelude::*;

#[component]
pub fn ActivityPulse(data: Vec<DailyCount>) -> Element {
    let max_val = data.iter().map(|d| d.count).max().unwrap_or(0).max(1);
    let height = 150.0;
    let width = 600.0;
    let padding = 20.0;

    let step_x = (width - padding * 2.0) / (data.len().max(1) as f32 - 1.0).max(1.0);
    let scale_y = (height - padding * 2.0) / max_val as f32;

    let points = data
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let x = padding + i as f32 * step_x;
            let y = height - padding - d.count as f32 * scale_y;
            format!("{},{}", x, y)
        })
        .collect::<Vec<_>>()
        .join(" ");

    let fill_points = format!(
        "{},{} {} {},{}",
        padding,
        height - padding,
        points,
        width - padding,
        height - padding
    );

    rsx! {
        div { class: "w-full space-y-2",
            div { class: "flex justify-between items-end",
                h4 { class: "text-[10px] font-bold text-cyan-500 uppercase tracking-[0.2em]", "Signal Strength (30D)" }
                span { class: "text-[10px] font-mono text-cyan-500/50", "MAX: {max_val} UNITS" }
            }
            div { class: "relative bg-cyan-500/5 border border-cyan-500/10 rounded overflow-hidden aspect-[4/1]",
                svg {
                    view_box: "0 0 {width} {height}",
                    class: "w-full h-full",
                    // Grid Lines
                    for i in 0..5 {
                        line {
                            x1: "0",
                            y1: "{padding + i as f32 * (height - padding * 2.0) / 4.0}",
                            x2: "{width}",
                            y2: "{padding + i as f32 * (height - padding * 2.0) / 4.0}",
                            stroke: "currentColor",
                            class: "text-cyan-500/10",
                            stroke_width: "1"
                        }
                    }
                    // Area Fill
                    polyline {
                        points: "{fill_points}",
                        fill: "url(#grade)",
                        class: "opacity-30"
                    }
                    // Trend Line
                    polyline {
                        points: "{points}",
                        fill: "none",
                        stroke: "#00f3ff",
                        stroke_width: "2",
                        stroke_linejoin: "round",
                        filter: "url(#glow)",
                        class: "transition-all duration-1000"
                    }

                    defs {
                        linearGradient { id: "grade", x1: "0%", y1: "0%", x2: "0%", y2: "100%",
                            stop { offset: "0%", stop_color: "#00f3ff", stop_opacity: "0.5" }
                            stop { offset: "100%", stop_color: "#00f3ff", stop_opacity: "0" }
                        }
                        filter { id: "glow",
                            feDropShadow { dx: "0", dy: "0", std_deviation: "3", flood_color: "#00f3ff" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn StatusDonut(data: Vec<StatusCount>) -> Element {
    let total: i64 = data.iter().map(|d| d.count).sum();
    let size = 200.0;
    let center = size / 2.0;
    let radius = 70.0;
    let stroke_width = 15.0;
    let circumference = 2.0 * std::f32::consts::PI * radius;

    let mut current_offset = 0.0;

    rsx! {
        div { class: "w-full flex flex-col items-center gap-4",
            div { class: "relative group",
                svg {
                    view_box: "0 0 {size} {size}",
                    class: "w-40 h-40 transform -rotate-90",
                    // Background Circle
                    circle {
                        cx: "{center}",
                        cy: "{center}",
                        r: "{radius}",
                        fill: "none",
                        stroke: "currentColor",
                        class: "text-white/5",
                        stroke_width: "{stroke_width}"
                    }

                    for d in data.iter() {
                        {
                            let percentage = d.count as f32 / total.max(1) as f32;
                            let dash_array = format!("{} {}", percentage * circumference, circumference);
                            let offset = -current_offset;
                            current_offset += percentage * circumference;

                            let color = match d.status.as_str() {
                                "Offer" | "Accepted" => "#10b981",
                                "Rejected" => "#ef4444",
                                "Interviewing" => "#f59e0b",
                                _ => "#00f3ff"
                            };

                            rsx! {
                                circle {
                                    cx: "{center}",
                                    cy: "{center}",
                                    r: "{radius}",
                                    fill: "none",
                                    stroke: "{color}",
                                    stroke_width: "{stroke_width}",
                                    stroke_dasharray: "{dash_array}",
                                    stroke_dashoffset: "{offset}",
                                    class: "transition-all duration-1000 hover:stroke-[20px] cursor-help",
                                    stroke_linecap: "butt",
                                }
                            }
                        }
                    }
                }
                // Center Text
                div { class: "absolute inset-0 flex flex-col items-center justify-center pointer-events-none",
                    span { class: "text-2xl font-bold text-white font-mono", "{total}" }
                    span { class: "text-[8px] text-gray-500 uppercase tracking-widest", "Total Cases" }
                }
            }

            // Legend
            div { class: "grid grid-cols-2 gap-x-4 gap-y-1 w-full max-w-[200px]",
                for d in data.iter() {
                    div { class: "flex items-center gap-2",
                        div {
                            class: "w-1.5 h-1.5 rounded-full",
                            style: format!("background: {}", match d.status.as_str() {
                                "Offer" | "Accepted" => "#10b981",
                                "Rejected" => "#ef4444",
                                "Interviewing" => "#f59e0b",
                                _ => "#00f3ff"
                            })
                        }
                        span { class: "text-[10px] text-gray-400 font-mono", "{d.status}" }
                        span { class: "text-[10px] text-white font-mono ml-auto", "{d.count}" }
                    }
                }
            }
        }
    }
}
