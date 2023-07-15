use dominator::{html, Dom};
use futures_signals::signal::SignalExt;

#[component(render_fn = card)]
pub struct Card {
    #[signal]
    pub child: Dom,
}

#[inline]
pub fn card(props: impl CardPropsTrait + 'static) -> Dom {
    let CardProps { child, apply } = props.take();

    html!("div", {
        .class("dmat-card")
        .apply_if(apply.is_some(), |dom| dom.apply(apply.unwrap()))
        .apply_if(child.is_some(), |d| d.child_signal(child.unwrap().map(Some)))
    })
}
