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
                div { class: "text-cyan-500/50 text-center py-8 font-mono text-sm tracking-widest uppercase",
                    "// NO SIGNAL DETECTED"
                }
            }
        } else {
            rsx! {
                div { class: "space-y-1",
                    for app in apps.iter().take(5) {
                        Link {
                            to: crate::Route::ApplicationDetail { id: app.id.to_string() },
                            class: "block group flex items-center justify-between p-4 border border-cyan-500/10 bg-cyan-500/5 hover:bg-cyan-500/10 hover:border-cyan-500/30 transition-all duration-300 rounded",
                            div { class: "flex items-center gap-4",
                                div { class: "w-2 h-2 rounded-full bg-cyan-500 shadow-[0_0_10px_#00f3ff]" }
                                div {
                                    p { class: "text-white font-bold tracking-wide", "{app.company}" }
                                    p { class: "text-xs text-cyan-500/60 font-mono mt-0.5", "{app.role}" }
                                }
                            }
                            div { class: "flex items-center gap-4",
                                if let Some(count) = app.comment_count {
                                    if count > 0 {
                                        span { class: "text-xs px-2 py-0.5 rounded bg-purple-500/20 text-purple-300 border border-purple-500/30", "💬 {count}" }
                                    }
                                }
                                span {
                                    class: "px-3 py-1 text-[10px] font-bold uppercase tracking-widest border transition-colors",
                                    style: match app.status.as_str() {
                                        "Offer" => "color: #10b981; border-color: #10b981; background: rgba(16,185,129,0.1);",
                                        "Rejected" => "color: #ef4444; border-color: #ef4444; background: rgba(239,68,68,0.1);",
                                        _ => "color: #00f3ff; border-color: #00f3ff; background: rgba(0,243,255,0.05);",
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
        rsx! { div { class: "animate-pulse text-cyan-500/50 font-mono text-sm", "INITIALIZING..." } }
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
                div { class: "lg:col-span-2 bg-black/40 border border-cyan-500/20 p-6",
                    ActivityPulse { data: s.daily_activity.clone() }
                }
                // Status Distribution (1/3)
                div { class: "bg-black/40 border border-emerald-500/20 p-6 flex flex-col items-center",
                    h4 { class: "text-[10px] font-bold text-emerald-500 uppercase tracking-[0.2em] mb-4 self-start", "Status Distribution" }
                    StatusDonut { data: s.status_distribution.clone() }
                }
            }
        },
        _ => {
            rsx! { div { class: "h-48 flex items-center justify-center text-cyan-500/30 font-mono animate-pulse", "CALIBRATING VISUALS..." } }
        }
    };

    rsx! {
        div { class: "max-w-7xl mx-auto space-y-8 pb-12",
            // Header
            div { class: "flex items-center justify-between",
                h2 { class: "text-3xl font-bold text-white tracking-widest", "SYSTEM OVERVIEW" }
                div { class: "flex gap-2",
                    div { class: "w-3 h-3 rounded-full bg-red-500 animate-pulse" }
                    span { class: "text-xs font-mono text-red-500/80 tracking-widest", "LIVE FEED" }
                }
            }

            // Stats Grid
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6",
                // Total Apps
                div { class: "relative overflow-hidden bg-black/40 border border-cyan-500/20 p-6 group hover:border-cyan-500/50 transition-colors duration-300",
                    div { class: "absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity",
                        span { class: "text-6xl", "📂" }
                    }
                    h3 { class: "text-cyan-500/60 text-xs font-bold uppercase tracking-[0.2em] mb-2", "Total Applications" }
                    p { class: "text-4xl font-bold text-white font-mono", "{total}" }
                    div { class: "h-1 w-full bg-cyan-500/10 mt-4 overflow-hidden",
                         div { class: "h-full bg-cyan-500 w-[70%]" } // Fake progress
                    }
                }

                // Success Rate
                div { class: "relative overflow-hidden bg-black/40 border border-emerald-500/20 p-6 group hover:border-emerald-500/50 transition-colors duration-300",
                    div { class: "absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity",
                        span { class: "text-6xl", "🚀" }
                    }
                    h3 { class: "text-emerald-500/60 text-xs font-bold uppercase tracking-[0.2em] mb-2", "Success Rate" }
                    p { class: "text-4xl font-bold text-white font-mono", "{success_rate}%" }
                     div { class: "h-1 w-full bg-emerald-500/10 mt-4 overflow-hidden",
                         div { class: "h-full bg-emerald-500", style: "width: {success_rate}%" }
                    }
                }

                 // Active Threads
                div { class: "relative overflow-hidden bg-black/40 border border-blue-500/20 p-6 group hover:border-blue-500/50 transition-colors duration-300",
                    div { class: "absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity",
                         span { class: "text-6xl", "⚡" }
                    }
                    h3 { class: "text-blue-500/60 text-xs font-bold uppercase tracking-[0.2em] mb-2", "Active Threads" }
                    p { class: "text-4xl font-bold text-white font-mono", "{active}" }
                     div { class: "h-1 w-full bg-blue-500/10 mt-4 overflow-hidden",
                         div { class: "h-full bg-blue-500 w-1/2 animate-pulse" }
                    }
                }

                // Total Comments
                div { class: "relative overflow-hidden bg-black/40 border border-purple-500/20 p-6 group hover:border-purple-500/50 transition-colors duration-300",
                    div { class: "absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity",
                         span { class: "text-6xl", "💬" }
                    }
                    h3 { class: "text-purple-500/60 text-xs font-bold uppercase tracking-[0.2em] mb-2", "Comms Intercepted" }
                    p { class: "text-4xl font-bold text-white font-mono", "{total_comments}" }
                     div { class: "h-1 w-full bg-purple-500/10 mt-4 overflow-hidden",
                         div { class: "h-full bg-purple-500 w-full animate-pulse" }
                    }
                }
            }

            // Visualizations Section
            {charts_view}

            // Split View: Activity & Comms
            div { class: "grid grid-cols-1 lg:grid-cols-3 gap-8",
                 // Recent Activity (2/3)
                div { class: "lg:col-span-2 bg-black/40 border border-white/10 overflow-hidden h-fit",
                    div { class: "px-6 py-4 border-b border-white/10 flex justify-between items-center bg-white/5",
                        div { class: "flex items-center gap-3",
                            span { class: "text-xl", "📡" }
                            h3 { class: "text-sm font-bold text-white uppercase tracking-widest", "Recent Activity Log" }
                        }
                        div { class: "text-xs font-mono text-gray-500", "SYNCED" }
                    }
                    div { class: "p-6",
                         {recent_activity}
                    }
                    div { class: "px-6 py-3 border-t border-white/10 bg-white/5 text-right",
                         Link {
                            to: "/admin/applications",
                            class: "text-xs font-bold text-cyan-500 hover:text-cyan-400 uppercase tracking-widest transition-colors",
                            "VIEW ALL DATA >>"
                        }
                    }
                }

                // Recent Comments (1/3)
                div { class: "bg-black/40 border border-purple-500/20 overflow-hidden h-fit",
                     div { class: "px-6 py-4 border-b border-purple-500/20 flex justify-between items-center bg-purple-500/5",
                        div { class: "flex items-center gap-3",
                            span { class: "text-xl", "📨" }
                            h3 { class: "text-sm font-bold text-white uppercase tracking-widest", "Incoming Data" }
                        }
                        div { class: "w-2 h-2 rounded-full bg-purple-500 animate-pulse" }
                    }
                    div { class: "p-4 max-h-[600px] overflow-y-auto custom-scrollbar",
                         {recent_comments_list}
                    }
                }
            }
        }
    }
}
