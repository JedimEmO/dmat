use dominator::{html, Dom};

use futures_signals::signal::SignalExt;
use futures_signals_utils::split_signal::split_signal;

#[component(render_fn = content_block)]
pub struct ContentBlock {
    #[signal]
    #[default(None)]
    pub title_section: Option<Dom>,
    #[signal]
    #[default(None)]
    pub media_section: Option<Dom>,
    #[signal]
    #[default(None)]
    pub supporting_section: Option<Dom>,
    #[signal]
    #[default(None)]
    pub footer_section: Option<Dom>,
}

pub fn content_block(props: impl ContentBlockPropsTrait + 'static) -> Dom {
    let ContentBlockProps {
        title_section,
        media_section,
        supporting_section,
        footer_section,
        apply,
    } = props.take();

    let (title_section, has_title_section) = split_signal(title_section, false, |v| v.is_some());
    let (media_section, has_media_section) = split_signal(media_section, false, |v| v.is_some());
    let (supporting_section, has_supporting_section) =
        split_signal(supporting_section, false, |v| v.is_some());
    let (footer_section, has_footer_section) = split_signal(footer_section, false, |v| v.is_some());

    html!("div", {
        .class("dmat-content-block")
        .apply_if(apply.is_some(), |d| d.apply(apply.unwrap())
        .class_signal("-with-title", has_title_section)
        .class_signal("-with-media", has_media_section)
        .class_signal("-with-supporting", has_supporting_section)
        .class_signal("-with-footer", has_footer_section))
        .child_signal(title_section.map(|v| v.map(|t| html!("div", {
            .class("title")
            .child(t)
        }))))
        .child_signal(media_section.map(|v| v.map(|t| html!("div", {
            .class("media")
            .child(t)
        }))))
        .child_signal(supporting_section.map(|v| v.map(|t| html!("div", {
            .class("supporting")
            .child(t)
        }))))
        .child_signal(footer_section.map(|v| v.map(|t| html!("div", {
            .class("footer")
            .child(t)
        }))))
    })
}
