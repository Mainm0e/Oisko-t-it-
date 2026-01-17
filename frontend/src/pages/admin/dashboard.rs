use crate::components::charts::{ActivityPulse, StatusDonut};
use dioxus::prelude::*;

const DATE_FMT: &str = "%Y-%m-%d %H:%M";

#[component]
pub fn AdminDashboard() -> Element {
    let applications = use_resource(move || async move {
        crate::services::application_service::list_applications().await
    });

    let comments_resource = use_resource(move || async move {
        crate::services::application_service::get_recent_comments().await
    });

    let stats_resource = use_resource(move || async move {
        crate::services::application_service::get_dashboard_stats().await
    });

    let stats = use_memo(move || {
        if let Some(Ok(apps)) = applications.read().as_ref() {
            let total = apps.len();
            let success_cnt = apps
                .iter()
                .filter(|a| a.status == "Offer" || a.status == "Accepted")
                .count();
            let active_cnt = apps
                .iter()
                .filter(|a| a.status != "Rejected" && a.status != "Offer" && a.status != "Accepted")
                .count();
            // Count total comments
            let total_comments: i64 = apps.iter().map(|a| a.comment_count.unwrap_or(0)).sum();

            let success_rate = if total > 0 {
                (success_cnt as f64 / total as f64 * 100.0) as u64
            } else {
                0
            };
            (total, success_rate, active_cnt, total_comments)
        } else {
            (0, 0, 0, 0)
        }
    });

    let (total, success_rate, active, total_comments) = stats();

    let recent_activity = if let Some(Ok(apps)) = applications.read().as_ref() {
        if apps.is_empty() {
            rsx! {
                div { class: "text-center py-8 font-mono text-[10px] tracking-widest uppercase opacity-30",
                    "// NO SIGNAL DETECTED"
                }
            }
        } else {
            rsx! {
                div { class: "space-y-2",
                    for app in apps.iter().take(5) {
                        Link {
                            to: crate::Route::ApplicationDetail { id: app.id.to_string() },
                            class: "block group flex items-center justify-between p-4 border rounded transition-all duration-300",
                            style: "background: var(--hover-bg); border-color: var(--glass-border);",
                            div { class: "flex items-center gap-4",
                                div { class: "w-1.5 h-1.5 rounded-full animate-pulse", style: "background: var(--accent-color); box-shadow: 0 0 10px var(--accent-glow);" }
                                div {
                                    p { class: "font-bold tracking-wide text-sm", style: "color: var(--text-color)", "{app.company}" }
                                    p { class: "text-[10px] font-mono mt-0.5 opacity-40 uppercase", style: "color: var(--accent-color)", "{app.role}" }
                                }
                            }
                            div { class: "flex items-center gap-4",
                                if let Some(count) = app.comment_count {
                                    if count > 0 {
                                        span {
                                            class: "text-[8px] px-2 py-0.5 rounded border font-black uppercase tracking-widest",
                                            style: "background: var(--accent-glow); color: var(--accent-color); border-color: var(--glass-border);",
                                            "💬 {count}"
                                        }
                                    }
                                }
                                span {
                                    class: "px-3 py-1 text-[8px] font-black uppercase tracking-widest border transition-colors rounded",
                                    style: match app.status.as_str() {
                                        "Offer" | "Accepted" => "color: white; border-color: var(--status-offer); background: var(--status-offer);",
                                        "Rejected" => "color: white; border-color: var(--status-rejected); background: var(--status-rejected);",
                                        "Interviewing" => "color: white; border-color: var(--status-interview); background: var(--status-interview);",
                                        _ => "color: var(--text-color); border-color: var(--glass-border); background: var(--hover-bg);",
                                    },
                                    "{app.status}"
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! { div { class: "animate-pulse font-mono text-[10px] opacity-40 uppercase tracking-widest", "INITIALIZING..." } }
    };

    let recent_comments_list = if let Some(Ok(comments)) = comments_resource.read().as_ref() {
        if comments.is_empty() {
            rsx! {
                div { class: "text-purple-500/50 text-center py-8 font-mono text-sm tracking-widest uppercase",
                    "// SILENCE"
                }
            }
        } else {
            rsx! {
                div { class: "space-y-4",
                    for comment in comments {
                        Link {
                            to: crate::Route::ApplicationDetail { id: comment.application_id.to_string() },
                            class: "block p-4 border-l-2 border-purple-500/50 bg-purple-500/5 hover:bg-purple-500/10 transition-colors",
                            div { class: "flex justify-between items-start mb-2",
                                div {
                                    p { class: "text-sm font-bold text-white", "{comment.visitor_name}" }
                                    p { class: "text-[10px] text-purple-300/60 font-mono", "{comment.company} :: {comment.role}" }
                                }
                                span { class: "text-[10px] font-mono text-gray-500", "{comment.created_at.format(DATE_FMT)}" }
                            }
                            p { class: "text-xs text-gray-400 leading-relaxed italic line-clamp-2", "\"{comment.content}\"" }
                        }
                    }
                }
            }
        }
    } else {
        rsx! { div { class: "animate-pulse text-purple-500/50 font-mono text-sm", "SCANNING FREQUENCIES..." } }
    };

    let charts_view = match stats_resource.read().as_ref() {
        Some(Ok(s)) => rsx! {
            div { class: "grid grid-cols-1 lg:grid-cols-3 gap-8",
                 // Activity Pulse (2/3)
                div { class: "lg:col-span-2 glass border p-6 rounded",
                    style: "border-color: var(--glass-border); background: var(--card-bg);",
                    ActivityPulse { data: s.daily_activity.clone() }
                }
                // Status Distribution (1/3)
                div { class: "glass border p-6 flex flex-col items-center rounded",
                    style: "border-color: var(--glass-border); background: var(--card-bg);",
                    h4 { class: "text-[10px] font-black uppercase tracking-[0.2em] mb-4 self-start opacity-60",
                        style: "color: var(--accent-color)",
                        "Status Distribution"
                    }
                    StatusDonut { data: s.status_distribution.clone() }
                }
            }
        },
        _ => {
            rsx! { div { class: "h-48 flex items-center justify-center font-mono animate-pulse opacity-20 uppercase tracking-[0.5em] text-xs", "CALIBRATING VISUALS..." } }
        }
    };

    rsx! {
        div { class: "max-w-7xl mx-auto space-y-8 pb-12",
            // Header
            div { class: "flex items-center justify-between border-b pb-6",
                style: "border-color: var(--glass-border);",
                h2 { class: "text-4xl font-black tracking-tighter uppercase",
                    style: "color: var(--text-color); text-shadow: 0 0 10px var(--accent-glow);",
                    "SYSTEM OVERVIEW"
                }
                div { class: "flex items-center gap-3 px-4 py-2 glass rounded-full",
                    style: "border-color: var(--glass-border);",
                    div { class: "w-2 h-2 rounded-full bg-red-500 animate-pulse" }
                    span { class: "text-[10px] font-mono text-red-500/80 tracking-[0.3em] font-black uppercase", "LIVE_FEED" }
                }
            }

            // Stats Grid
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6",
                // Total Apps
                div { class: "relative overflow-hidden noir-card p-6 group",
                    div { class: "absolute top-4 right-4 text-4xl opacity-10 group-hover:opacity-20 transition-opacity", "📂" }
                    h3 { class: "text-[10px] font-black uppercase tracking-[0.2em] mb-2 opacity-60", "Total Applications" }
                    p { class: "text-4xl font-bold font-mono", style: "color: var(--text-color)", "{total}" }
                    div { class: "h-1 w-full bg-white/5 mt-6 overflow-hidden rounded-full",
                         div { class: "h-full bg-accent-color", style: "width: 70%; box-shadow: 0 0 10px var(--accent-glow);" }
                    }
                }

                // Success Rate
                div { class: "relative overflow-hidden noir-card p-6 group",
                    div { class: "absolute top-4 right-4 text-4xl opacity-10 group-hover:opacity-20 transition-opacity", "🚀" }
                    h3 { class: "text-[10px] font-black uppercase tracking-[0.2em] mb-2", style: "color: var(--status-offer)", "Success Rate" }
                    p { class: "text-4xl font-bold font-mono", style: "color: var(--status-offer)", "{success_rate}%" }
                     div { class: "h-1 w-full bg-white/5 mt-6 overflow-hidden rounded-full",
                         div { class: "h-full", style: "width: {success_rate}%; background: var(--status-offer); box-shadow: 0 0 10px var(--status-offer);" }
                    }
                }

                 // Active Threads
                div { class: "relative overflow-hidden noir-card p-6 group",
                    div { class: "absolute top-4 right-4 text-4xl opacity-10 group-hover:opacity-20 transition-opacity", "⚡" }
                    h3 { class: "text-[10px] font-black uppercase tracking-[0.2em] mb-2", style: "color: var(--status-interview)", "Active Threads" }
                    p { class: "text-4xl font-bold font-mono", style: "color: var(--status-interview)", "{active}" }
                     div { class: "h-1 w-full bg-white/5 mt-6 overflow-hidden rounded-full",
                         div { class: "h-full animate-pulse", style: "width: 50%; background: var(--status-interview); box-shadow: 0 0 10px var(--status-interview);" }
                    }
                }

                // Total Comments
                div { class: "relative overflow-hidden noir-card p-6 group",
                    div { class: "absolute top-4 right-4 text-4xl opacity-10 group-hover:opacity-20 transition-opacity", "💬" }
                    h3 { class: "text-[10px] font-black uppercase tracking-[0.2em] mb-2 opacity-60", "Comms Intercepted" }
                    p { class: "text-4xl font-bold font-mono", style: "color: var(--text-color)", "{total_comments}" }
                     div { class: "h-1 w-full bg-white/5 mt-6 overflow-hidden rounded-full",
                         div { class: "h-full animate-pulse", style: "width: 100%; background: var(--accent-color); box-shadow: 0 0 10px var(--accent-glow);" }
                    }
                }
            }

            // Visualizations Section
            {charts_view}

            // Split View: Activity & Comms
            div { class: "grid grid-cols-1 lg:grid-cols-3 gap-8",
                 // Recent Activity (2/3)
                div { class: "lg:col-span-2 glass border overflow-hidden h-fit rounded",
                    style: "border-color: var(--glass-border); background: var(--card-bg);",
                    div { class: "px-6 py-4 border-b flex justify-between items-center bg-white/5",
                        style: "border-color: var(--glass-border);",
                        div { class: "flex items-center gap-3",
                            span { class: "text-xl", "📡" }
                            h3 { class: "text-[10px] font-black uppercase tracking-[0.2em] opacity-80", style: "color: var(--text-color)", "Recent Activity Log" }
                        }
                        div { class: "text-[8px] font-mono opacity-40 uppercase tracking-widest", "SYNCED" }
                    }
                    div { class: "p-6",
                         {recent_activity}
                    }
                    div { class: "px-6 py-4 border-t bg-white/5 text-right",
                        style: "border-color: var(--glass-border);",
                         Link {
                            to: "/admin/applications",
                            class: "text-[10px] font-black uppercase tracking-[0.2em] transition-colors",
                            style: "color: var(--accent-color)",
                            "VIEW ALL DATA >>"
                        }
                    }
                }

                // Recent Comments (1/3)
                div { class: "glass border overflow-hidden h-fit rounded",
                    style: "border-color: var(--glass-border); background: var(--card-bg);",
                     div { class: "px-6 py-4 border-b flex justify-between items-center bg-white/5",
                        style: "border-color: var(--glass-border);",
                        div { class: "flex items-center gap-3",
                            span { class: "text-xl", "📨" }
                            h3 { class: "text-[10px] font-black uppercase tracking-[0.2em] opacity-80", style: "color: var(--text-color)", "Incoming Data" }
                        }
                        div { class: "w-1.5 h-1.5 rounded-full animate-pulse", style: "background: var(--accent-color); box-shadow: 0 0 10px var(--accent-glow);" }
                    }
                    div { class: "p-4 max-h-[600px] overflow-y-auto custom-scrollbar",
                         {recent_comments_list}
                    }
                }
            }
        }
    }
}
