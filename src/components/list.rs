use dominator::{html, Dom};
use futures_signals::signal_vec::{SignalVec, SignalVecExt};

pub fn list<T: SignalVec<Item = Dom> + 'static>(children: T) -> Dom {
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
