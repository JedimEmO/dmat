use crate::components::mixins::disabled_signal_mixin;
use crate::utils::signals::mutation::store_signal_value_mixin;
use dmat_utils::svg::animated_attribute::animated_attribute;
use dominator::{clone, events, html, svg, Dom, DomBuilder};
use futures::channel::mpsc::{channel, Receiver};
use futures_signals::signal::{Mutable, Signal};
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlElement;

#[macro_export]
macro_rules! switch {
    ($props: expr) => {{
        $crate::components::input::switch::switch($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::input::switch::switch($props, $mixin)
    }};
}

pub struct SwitchProps<
    TStateSignal: FnMut() -> Box<dyn Signal<Item = bool> + Unpin>,
    TDisabledSignal: Signal<Item = bool> + Unpin,
> {
    pub state_signal: TStateSignal,
    pub disabled_signal: TDisabledSignal,
}

pub struct SwitchOut {
    pub toggle_stream: Receiver<()>,
}

pub fn switch<TStateSignal, TDisabledSignal, F>(
    props: SwitchProps<TStateSignal, TDisabledSignal>,
    mixin: F,
) -> (Dom, SwitchOut)
where
    TStateSignal: FnMut() -> Box<dyn Signal<Item = bool> + Unpin> + 'static,
    TDisabledSignal: Signal<Item = bool> + Unpin + 'static,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let (mut toggle_tx, toggle_stream) = channel(1);
    let disabled_signal = props.disabled_signal;
    let mut state_signal_fn = props.state_signal;
    let is_disabled = Mutable::new(false);

    (
        html!("div", {
            .apply(mixin)
            .class("dmat-switch")
            .class_signal("on", state_signal_fn())
            .apply(store_signal_value_mixin(disabled_signal, &is_disabled))
            .apply(disabled_signal_mixin(is_disabled.signal_cloned()))
            .style("height", "1rem")
            .style("width", "2rem")
            .event(clone!(is_disabled => {
                move |e: events::Click| {
                    if !is_disabled.get() {
                        e.stop_propagation();
                        toggle_tx.try_send(()).unwrap_throw();
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
                            animated_attribute(b, state_signal_fn(), Rc::new(|v| {
                                match v {
                                    true => "75".to_string(),
                                    _ => "25".to_string()
                                }
                            }), "cx".to_string(), Duration::from_millis(20))
                        })
                    })
                ])
            }))
        }),
        SwitchOut { toggle_stream },
    )
}
