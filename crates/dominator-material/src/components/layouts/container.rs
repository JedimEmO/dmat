use crate::elements::new_html::new_html;
use crate::utils::component_signal::ComponentSignal;
use dominator::DomBuilder;
use web_sys::HtmlElement;

pub fn container<T: Into<ComponentSignal>>(child: T) -> DomBuilder<HtmlElement> {
    new_html("div")
        .class("dmat-container")
        .child_signal(child.into().0)
}
