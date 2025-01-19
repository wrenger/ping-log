use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct PingStats {
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub lost: usize,
    pub count: usize,
    pub time: i64,
}

#[component]
pub fn Stats(stats: PingStats) -> Element {
    rsx! {
        div { class: "card bg-neutral shadow-xl container mx-auto",
            div { class: "card-body",
                div { class: "stats",
                    div { class: "stat",
                        div { class: "stat-title", "Min" }
                        div { class: "stat-value", "{stats.min:.1} ms" }
                        div { class: "stat-desc", "last hour" }
                    }
                    div { class: "stat",
                        div { class: "stat-title", "Avg" }
                        div { class: "stat-value", "{stats.avg:.1} ms" }
                        div { class: "stat-desc", "{stats.count} samples" }
                    }
                    div { class: "stat",
                        div { class: "stat-title", "Max" }
                        div { class: "stat-value", "{stats.max:.1} ms" }
                        div { class: "stat-desc", "{stats.lost} timeouts" }
                    }
                }
            }
        }
    }
}
