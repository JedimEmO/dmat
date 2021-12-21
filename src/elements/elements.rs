use dominator::DomBuilder;
use web_sys::Element;

#[inline]
pub fn new_html(node: &str) -> DomBuilder<Element> {
    DomBuilder::new_html(node)
}
