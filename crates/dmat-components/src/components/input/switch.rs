use crate::components::mixins::disabled_signal_mixin;
use dominator::{html, svg, Dom, DomBuilder};
use futures::channel::mpsc::{channel, Receiver};
use futures_signals::signal::Signal;
use web_sys::HtmlElement;

pub struct SwitchProps<TStateSignal: Signal<Item = bool>> {
    pub state_signal: TStateSignal,
}

pub struct SwitchOut {
    pub toggle_stream: Receiver<()>,
}

pub fn switch<TStateSignal, F>(props: SwitchProps<TStateSignal>, mixin: F) -> (Dom, SwitchOut)
where
    TStateSignal: Signal<Item = bool> + Unpin + 'static,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let (mut toggle_tx, toggle_stream) = channel(1);

    (
        html!("div", {
            .apply(mixin)
            .apply(disabled_signal_mixin(props.state_signal))
            .style("height", "1rem")
            .child(svg!("svg", {
                .attribute("viewBox", "0 0 100 100")
                .children(&mut [
                    svg!("rect", {
                        .attribute("y", "20")
                        .attribute("width", "200")
                        .attribute("height", "20")
                    }),
                    svg!("circle", {
                        .attribute("cx", "100")
                        .attribute("cy", "50")
                        .attribute("r", "50")
                    })
                ])
            }))
        }),
        SwitchOut { toggle_stream },
    )
}
