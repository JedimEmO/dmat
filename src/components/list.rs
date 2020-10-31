use dominator::{html, Dom};
use futures_signals::signal_vec::{SignalVec, SignalVecExt};

pub struct List {}

impl List {
    #[inline]
    pub fn new_static<A: IntoIterator<Item = Dom>>(children: A) -> Dom {
        html!("ul", {
            .class("dmat-list")
            .children(children.into_iter().map(|child| {
                 html!("li", {
                    .class("dmat-list-item")
                    .child(child)
                })
            }))
        })
    }

    pub fn new_dynamic<A>(children: A) -> Dom
    where
        A: SignalVec<Item = Dom> + 'static,
    {
        html!("ul", {
            .class("dmat-list")
            .children_signal_vec(children.map(|child| {
                 html!("li", {
                    .class("dmat-list-item")
                    .child(child)
                })
            }))
        })
    }
}
