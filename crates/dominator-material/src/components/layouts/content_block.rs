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
        .children(vec![
            props.title_section,
            props.media_section,
            props.supporting_section,
            props.footer_section
        ].into_iter().filter_map(|v| v))
    })
}
