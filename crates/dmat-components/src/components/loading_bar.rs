use dominator::{html, Dom};
use wasm_bindgen::__rt::core::time::Duration;

#[component(render_fn = loading_bar)]
pub struct LoadingBar {
    #[default(std::time::Duration::from_secs(1))]
    pub duration: Duration,
    #[default(ProgressIndicatorIterations::Infinite)]
    pub iterations: ProgressIndicatorIterations,
}

pub enum ProgressIndicatorIterations {
    Infinite,
    Count(usize),
}

pub fn loading_bar(props: impl LoadingBarPropsTrait + 'static) -> Dom {
    let LoadingBarProps {
        duration,
        iterations,
        apply,
    } = props.take();

    let animation_iterations = match iterations {
        ProgressIndicatorIterations::Infinite => "infinite".into(),
        ProgressIndicatorIterations::Count(count) => format!("{}", count),
    };

    let animation_duration = format!("{}s", duration.as_secs_f32());

    html!("div", {
        .class("dmat-progress-indicator")
        .apply_if(apply.is_some(), |d| d.apply(apply.unwrap()))
        .child(html!("div", {
            .class("dmat-progress-bar")
            .style("animation-duration", animation_duration.as_str())
            .style("animation-iteration-count", animation_iterations.as_str())
        }))
    })
}
