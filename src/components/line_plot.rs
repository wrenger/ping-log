use dioxus::prelude::*;
use dioxus_elements::geometry::euclid::{Point2D, Rect, Size2D};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Line {
    pub color: String,
    pub points: Vec<(f64, f64)>,
}
impl Line {
    fn path(&self, to_cord: impl Fn((f64, f64)) -> (f64, f64)) -> String {
        let cords = self
            .points
            .iter()
            .cloned()
            .map(to_cord)
            .map(|p| format!("{} {}", p.0, p.1))
            .collect::<Vec<_>>();
        format!("M {}", cords.join(" L "))
    }
}

const SPACE_FOR_TICK: f64 = 35.0;

#[component]
pub fn LinePlot(
    lines: Vec<Line>,
    #[props(default = 5.0)] padding: f64,
    #[props(default = 180.0)] height: f64,
    #[props(default = 400.0)] width: f64,
    #[props(default = 15.0)] axis_padding: f64,
    #[props(default = 3.0)] tick_size: f64,
    min_x: Option<f64>,
    max_x: Option<f64>,
    #[props(default = 10.0)] x_gran: f64,
    x_trans: Option<Callback<f64, String>>,
    min_y: Option<f64>,
    max_y: Option<f64>,
    #[props(default = 10.0)] y_gran: f64,
    y_trans: Option<Callback<f64, String>>,
) -> Element {
    let bb = Rect::<f64, ()>::new(
        Point2D::new(axis_padding + padding, padding),
        Size2D::new(
            width - axis_padding - 2.0 * padding,
            height - axis_padding - 2.0 * padding,
        ),
    );

    let x_ax = Axis::new(
        min_x,
        max_x,
        lines.iter().flat_map(|l| l.points.iter()).map(|p| p.0),
        2.0 * SPACE_FOR_TICK,
        x_gran,
        bb.width(),
    );
    let y_ax = Axis::new(
        min_y,
        max_y,
        lines.iter().flat_map(|l| l.points.iter()).map(|p| p.1),
        SPACE_FOR_TICK,
        y_gran,
        bb.height(),
    );

    let to_cord = |p: (f64, f64)| {
        (
            bb.min_x() + (p.0 - x_ax.min) / x_ax.width() * bb.width(),
            bb.max_y() - (p.1 - y_ax.min) / y_ax.width() * bb.height(),
        )
    };

    let x_trans = |v: f64| {
        x_trans
            .as_ref()
            .map(|cb| cb.call(v))
            .unwrap_or(v.to_string())
    };
    let y_trans = |v: f64| {
        y_trans
            .as_ref()
            .map(|cb| cb.call(v))
            .unwrap_or(v.to_string())
    };

    rsx! {
        svg { class: "w-full",
            view_box: "0 0 {width} {height}",
            xmlns: "http://www.w3.org/2000/svg",
            rect {
                x: bb.min_x(),
                y: bb.min_y(),
                width: bb.width(),
                height: bb.height(),
                fill: "oklch(var(--bg))",
            }
            // Grid
            g {
                name: "grid",
                stroke: "oklch(var(--bc)/0.5)",
                "stroke-width": "0.5",
                for t in x_ax.ticks.iter().map(|&t| to_cord((t, 0.0)).0) {
                    path { d: "M {t} {bb.min_y()} L {t} {bb.max_y()}" }
                }
                for t in y_ax.ticks.iter().map(|&t| to_cord((0.0, t)).1) {
                    path { d: "M {bb.min_x()} {t} L {bb.max_x()} {t}" }
                }
            }
            // Lines
            for line in &lines {
                path {
                    d: line.path(to_cord),
                    stroke: "{line.color}",
                    "stroke-width": "1",
                    fill: "none",
                }
                g { fill: "{line.color}",
                    for (v, p) in line.points.iter().map(|&v| (v, to_cord(v))) {
                        circle { cx: p.0, cy: p.1, r: 2.0,
                            title { "{x_trans(v.0)} {y_trans(v.1)}" }
                        }
                    }
                }
            }
            // Axes
            g {
                name: "x-axis",
                font_size: "0.6em",
                text_anchor: "middle",
                dominant_baseline: "hanging",
                fill: "oklch(var(--bc))",
                path {
                    d: "M {bb.min_x()} {bb.max_y()} L {bb.max_x()} {bb.max_y()}",
                    stroke: "oklch(var(--bc))",
                    "stroke-width": "1",
                }
                for (t , val) in x_ax.ticks.iter().map(|&t| (t, to_cord((t, 0.0)).0)) {
                    text { x: val, y: "{bb.max_y() + 2.0 * tick_size}", "{x_trans(t)}" }
                    path {
                        d: "M {val} {bb.max_y()} L {val} {bb.max_y() + tick_size}",
                        stroke: "oklch(var(--bc))",
                        "stroke-width": "1",
                    }
                }
            }
            g {
                name: "y-axis",
                font_size: "0.6em",
                text_anchor: "end",
                dominant_baseline: "middle",
                fill: "oklch(var(--bc))",
                path {
                    d: "M {bb.min_x()} {bb.max_y()} L {bb.min_x()} {bb.min_y()}",
                    stroke: "oklch(var(--bc))",
                    "stroke-width": "1",
                }
                for (t , val) in y_ax.ticks.iter().map(|&t| (t, to_cord((0.0, t)).1)) {
                    text { x: "{bb.min_x() - 2.0 * tick_size}", y: val, "{y_trans(t)}" }
                    path {
                        d: "M {bb.min_x() - tick_size} {val} L {bb.min_x()} {val}",
                        stroke: "oklch(var(--bc))",
                        "stroke-width": "1",
                    }
                }
            }
        }
    }
}

struct Axis {
    min: f64,
    max: f64,
    ticks: Vec<f64>,
}

impl Axis {
    fn new(
        min: Option<f64>,
        max: Option<f64>,
        values: impl Iterator<Item = f64> + Clone,
        ticks_space: f64,
        ticks_gran: f64,
        size: f64,
    ) -> Self {
        let min = match min {
            Some(x) => x,
            None => values.clone().fold(f64::INFINITY, f64::min),
        };
        let max = match max {
            Some(x) => x,
            None => values.clone().fold(f64::NEG_INFINITY, f64::max),
        };
        let min = if min.is_finite() { min } else { 0.0 };
        let max = if max.is_finite() { max } else { 1.0 };

        let width = (max - min).max(f64::MIN_POSITIVE);

        // Number of ticks that have enough space to be readable
        let fitting_ticks = (size / ticks_space) as usize;
        // Number of ticks that could be placed
        let potential_ticks = (width / ticks_gran) as usize;
        let ticks = if fitting_ticks > 0 && potential_ticks > 0 {
            let ticks_start = (min / ticks_gran).ceil() * ticks_gran;
            (0..potential_ticks)
                .map(|i| ticks_start + i as f64 * ticks_gran)
                .step_by((potential_ticks / fitting_ticks).max(1))
                .collect::<Vec<_>>()
        } else {
            vec![min]
        };

        Self { min, max, ticks }
    }

    fn width(&self) -> f64 {
        (self.max - self.min).max(1.0)
    }
}
