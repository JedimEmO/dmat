use dominator::{html, Dom, DomBuilder};
use futures_signals::signal_vec::{always, SignalVec, SignalVecExt};
use web_sys::HtmlElement;

#[inline]
pub fn list<
    T: SignalVec<Item = Dom> + 'static,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
>(
    children: T,
    mixin: F,
) -> Dom {
    html!("ul", {
        .class("dmat-list")
        .apply(mixin)
        .children_signal_vec(children.map(|child| {
             html!("li", {
                .class("dmat-list-item")
                .child(child)
            })
        }))
    })
}

#[inline]
pub fn static_list<F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>(
    children: Vec<Dom>,
    mixin: F,
) -> Dom {
    list(always(children), mixin)
}
