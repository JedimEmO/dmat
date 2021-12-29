use crate::utils::component_signal::ComponentSignal;
use dominator::{html, Dom};

pub fn container<T: Into<ComponentSignal>>(child: T) -> Dom {
    html!("div", {
        .class("dmat-container")
        .child_signal(child.into().0)
    })
}
