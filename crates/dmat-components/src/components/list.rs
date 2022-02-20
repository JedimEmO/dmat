use dominator::{html, Dom, DomBuilder};
use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use web_sys::HtmlElement;

#[macro_export]
macro_rules! list {
    ($props: expr) => {{
        $crate::components::list::list($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::list::list($props, $mixin)
    }};
}

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

#[macro_export]
macro_rules! static_list {
    ($props: expr) => {{
        $crate::components::list::static_list($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::list::static_list($props, $mixin)
    }};
}

#[inline]
pub fn static_list<TChildren, F>(children: TChildren, mixin: F) -> Dom
where
    TChildren: IntoIterator<Item = Dom>,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("ul", {
        .class("dmat-list")
        .class("-static")
        .apply(mixin)
        .children(children.into_iter().map(|child| {
             html!("li", {
                .class("dmat-list-item")
                .child(child)
            })
        }))
    })
}
