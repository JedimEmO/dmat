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
    let (mut _toggle_tx, toggle_stream) = channel(1);

    (
        html!("div", {
            .apply(mixin)
            .apply(disabled_signal_mixin(props.state_signal))
            .style("height", "1rem")
            .child(svg!("svg", {
                .attr("viewBox", "0 0 100 100")
                .children(&mut [
                    svg!("rect", {
                        .attr("y", "20")
                        .attr("width", "200")
                        .attr("height", "20")
                    }),
                    svg!("circle", {
                        .attr("cx", "100")
                        .attr("cy", "50")
                        .attr("r", "50")
                    })
                ])
            }))
        }),
        SwitchOut { toggle_stream },
    )
}
