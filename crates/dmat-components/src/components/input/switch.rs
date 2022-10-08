use dmat_utils::svg::animated_attribute::animated_attribute;
use dominator::{events, html, svg, Dom, DomBuilder};
use futures::channel::mpsc::{channel, Receiver};
use futures_signals::signal::Signal;
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

pub struct SwitchProps<TStateSignal: FnMut() -> Box<dyn Signal<Item = bool> + Unpin>> {
    pub state_signal: TStateSignal,
}

pub struct SwitchOut {
    pub toggle_stream: Receiver<()>,
}

pub fn switch<TStateSignal, F>(mut props: SwitchProps<TStateSignal>, mixin: F) -> (Dom, SwitchOut)
where
    TStateSignal: FnMut() -> Box<dyn Signal<Item = bool> + Unpin> + 'static,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let (mut toggle_tx, toggle_stream) = channel(1);

    (
        html!("div", {
            .apply(mixin)
            .class("dmat-switch")
            .class_signal("on", (props.state_signal)())
            .style("height", "1rem")
            .style("width", "2rem")
            .event(move |e: events::Click| {
                e.stop_propagation();
                toggle_tx.try_send(()).unwrap_throw();
            })
            .child(svg!("svg", {
                .attr("viewBox", "0 0 100 50")
                .children(&mut [
                    svg!("path", {
                        .class("slit")
                        .attr("d", "M 20 5 \
                        A 20 20 0 0 0 20 45 \
                        L 80 45 \
                        A 20 20 0 0 0 80 5")
                    }),
                    svg!("circle", {
                        .class("knob")
                        .attr("cy", "25")
                        .attr("r", "25")
                        .apply(|b| {
                            animated_attribute(b, (props.state_signal)(), Rc::new(|v| {
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
