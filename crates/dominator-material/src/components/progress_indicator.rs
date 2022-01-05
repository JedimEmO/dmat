use dominator::{html, Dom, DomBuilder};
use wasm_bindgen::__rt::core::time::Duration;
use web_sys::HtmlElement;

pub enum ProgressIndicatorIterations {
    Infinite,
    Count(usize),
}

#[inline]
pub fn progress_indicator<F>(
    duration: Duration,
    iterations: ProgressIndicatorIterations,
    mixin: F,
) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let animation_iterations = match iterations {
        ProgressIndicatorIterations::Infinite => "infinite".into(),
        ProgressIndicatorIterations::Count(count) => format!("{}", count),
    };

    let animation_duration = format!("{}s", duration.as_secs_f32());

    html!("div", {
        .class("dmat-progress-indicator")
        .apply(mixin)
        .child(html!("div", {
            .class("dmat-progress-bar")
            .style("animation-duration", animation_duration.as_str())
            .style("animation-iteration-count", animation_iterations.as_str())
        }))
    })
}
