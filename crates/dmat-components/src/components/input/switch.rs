use crate::components::mixins::disabled_signal_mixin;
use crate::utils::signals::mutation::store_signal_value_mixin;
use dmat_utils::svg::animated_attribute::animated_attribute;
use dominator::{clone, events, html, svg, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use std::time::Duration;

#[component(render_fn = switch)]
pub struct Switch<TClickHandler: Fn(events::Click) = fn(events::Click) -> ()> {
    #[signal]
    #[default(false)]
    pub state: bool,
    #[signal]
    #[default(false)]
    pub disabled: bool,
    #[default(|_| {})]
    pub click_handler: TClickHandler,
}

pub fn switch(props: impl SwitchPropsTrait + 'static) -> Dom {
    let SwitchProps {
        state,
        disabled,
        click_handler,
        apply,
    } = props.take();

    let state_bc = state.broadcast();
    let disabled_bc = disabled.broadcast();
    let is_disabled = Mutable::new(false);

    html!("div", {
        .apply_if(apply.is_some(), |dom| { apply.unwrap()(dom) })
        .class("dmat-switch")
        .class_signal("on", state_bc.signal())
        .apply(store_signal_value_mixin(disabled_bc.signal(), &is_disabled))
        .apply(disabled_signal_mixin(disabled_bc.signal()))
        .style("height", "1rem")
        .style("width", "2rem")
        .event(clone!(is_disabled => {
            move |e: events::Click| {
                if !is_disabled.get() {
                    click_handler(e);
                }
            }
        }))
        .child(svg!("svg", {
            .attr("viewBox", "0 0 100 50")
            .children(&mut [
                svg!("path", {
                    .class("track")
                    .attr("d", "M 20 5 \
                    A 20 20 0 0 0 20 45 \
                    L 80 45 \
                    A 20 20 0 0 0 80 5")
                }),
                svg!("circle", {
                    .class("thumb")
                    .attr("cy", "25")
                    .attr("r", "25")
                    .apply(|b| {
                        animated_attribute(b, state_bc.signal(), Box::new(|v| {
                            match v {
                                true => "75".to_string(),
                                _ => "25".to_string()
                            }
                        }), "cx".to_string(), Duration::from_millis(20))
                    })
                })
            ])
        }))
    })
}
