use dominator::{html, Dom, DomBuilder};
use web_sys::HtmlElement;

#[macro_export]
macro_rules! content_block {
    ($a: expr) => {{
        $crate::components::layouts::content_block($a, |d| d)
    }};

    ($a: expr, $mixin: expr) => {{
        $crate::components::layouts::content_block($a, $mixin)
    }};
}

#[derive(Default)]
pub struct ContentBlockProps {
    pub title_section: Option<Dom>,
    pub media_section: Option<Dom>,
    pub supporting_section: Option<Dom>,
    pub footer_section: Option<Dom>,
}

pub fn content_block<F>(props: ContentBlockProps, mixin: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("div", {
        .class("dmat-content-block")
        .apply(mixin)
        .apply_if(props.title_section.is_some(), |d| d.class("-with-title"))
        .apply_if(props.media_section.is_some(), |d| d.class("-with-media"))
        .apply_if(props.supporting_section.is_some(), |d| d.class("-with-supporting"))
        .apply_if(props.footer_section.is_some(), |d| d.class("-with-footer"))
        .children(vec![
            props.title_section.map(|d| html!("div", { .child(d).class("title")})),
            props.media_section.map(|d| html!("div", { .child(d).class("media")})),
            props.supporting_section.map(|d| html!("div", { .child(d).class("supporting")})),
            props.footer_section.map(|d| html!("div", { .child(d).class("footer")})),
        ].into_iter().flatten())
    })
}
