use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Describes the system status of the underlaying linux server.
#[derive(Props, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct HardwareProps {
    /// CPU load in percent times the number of CPUs.
    pub load: f32,
    /// Current memory consumption.
    pub memory_used: f32,
    /// Total memory installed on the system.
    pub memory_total: f32,
    /// CPU temperature.
    pub temperature: f32,
}

pub fn Hardware(props: HardwareProps) -> Element {
    rsx! {

        div { class: "card bg-neutral shadow-xl container mx-auto",
            div { class: "card-body",
                div { class: "stats",
                    div { class: "stat",
                        div { class: "stat-title", "CPU Load" }
                        div { class: "stat-value", "{props.load:.2} %" }
                        div { class: "stat-desc", "last minute" }
                    }
                    div { class: "stat",
                        div { class: "stat-title", "Memory" }
                        div { class: "stat-value", "{props.memory_used:.1} GB" }
                        div { class: "stat-desc", "of {props.memory_total:.1} GB" }
                    }
                    div { class: "stat",
                        div { class: "stat-title", "Temperature" }
                        div { class: "stat-value", "{props.temperature:.1}°C" }
                        div { class: "stat-desc", "zone 0" }
                    }
                }
            }
        }
    }
}
