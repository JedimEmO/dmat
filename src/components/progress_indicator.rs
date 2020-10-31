use dominator::{html, Dom};
use wasm_bindgen::__rt::core::time::Duration;

pub enum ProgressIndicatorIterations {
    Infinite,
    Count(usize),
}

pub struct ProgressIndicator {}

impl ProgressIndicator {
    pub fn new(duration: Duration, iterations: ProgressIndicatorIterations) -> Dom {
        let animation_iterations = match iterations {
            ProgressIndicatorIterations::Infinite => "infinite".into(),
            ProgressIndicatorIterations::Count(count) => format!("{}", count),
        };

        let animation_duration = format!("{}s", duration.as_secs_f32());

        html!("div", {
            .class("dmat-progress-indicator")
            .child(html!("div", {
                .class("dmat-progress-bar")
                .style("animation-duration", animation_duration.as_str())
                .style("animation-iteration-count", animation_iterations.as_str())
            }))
        })
    }
}
