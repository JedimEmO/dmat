use dominator::{Dom, html};
use wasm_bindgen::__rt::core::time::Duration;

#[derive(Clone)]
pub enum ProgressIndicatorIterations {
    Infinite,
    Count(usize),
}

#[derive(Clone)]
pub struct ProgressIndicator {
    duration: Duration,
    iterations: ProgressIndicatorIterations
}

impl ProgressIndicator {
    pub fn build(duration: Duration) -> ProgressIndicator {
        ProgressIndicator {
            duration,
            iterations: ProgressIndicatorIterations::Infinite
        }
    }

    pub fn iterations(mut self, iterations: ProgressIndicatorIterations) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn dom(self) -> Dom {
        progress_indicator(self)
    }
}

fn progress_indicator(progress_indicator: ProgressIndicator) -> Dom {
    Dom::with_state(progress_indicator, |progress_indicator| {
        let animation_iterations = match progress_indicator.iterations {
            ProgressIndicatorIterations::Infinite => "infinite".into(),
            ProgressIndicatorIterations::Count(count) => {
                format!("{}", count)
            }
        };

        let animation_duration = format!("{}s", progress_indicator.duration.as_secs_f32());

        html!("div", {
            .class("dmat-progress-indicator")
            .child(html!("div", {
                .class("dmat-progress-bar")
                .style("animation-duration", animation_duration.as_str())
                .style("animation-iteration-count", animation_iterations.as_str())
            }))
        })
    })
}
