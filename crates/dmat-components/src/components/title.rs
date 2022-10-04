use dominator::{html, Dom, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::{Signal, SignalExt};
use web_sys::HtmlElement;

#[macro_export]
macro_rules! title {
    ($props: expr) => {{
        $crate::components::title::title($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::title::title($props, $mixin)
    }};
}

pub struct TitleProps<
    THeaderTextSignal: Signal<Item = String>,
    TSubHeaderTextSignal: Signal<Item = Option<String>>,
> {
    pub header_text_signal: THeaderTextSignal,
    pub sub_header_text_signal: TSubHeaderTextSignal,
}

pub fn title<THeaderTextSignal, TSubHeaderTextSignal, F>(
    props: TitleProps<THeaderTextSignal, TSubHeaderTextSignal>,
    mixin: F,
) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    THeaderTextSignal: Signal<Item = String> + 'static,
    TSubHeaderTextSignal: Signal<Item = Option<String>> + 'static,
{
    let children = map_ref! {
        let header = props.header_text_signal,
        let sub_header = props.sub_header_text_signal => move {
            vec![
                Some(html!("div", {
                    .class("title")
                    .text(header)
                })),
                sub_header.as_ref().map(move |v| html!("div", {
                    .class("sub-title")
                    .text(v)
                }))
            ].into_iter().flatten().collect()
        }
    }
    .to_signal_vec();

    html!("div",{
        .class("dmat-title")
        .apply(mixin)
        .children_signal_vec(children)
    })
}
