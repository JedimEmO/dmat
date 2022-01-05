use crate::utils::component_signal::{ComponentSignal, DomOption};
use dominator::{html, Dom, DomBuilder};
use futures_signals::signal::Signal;
use web_sys::HtmlElement;

#[derive(Default)]
pub struct CardProps {
    pub header_view: Option<ComponentSignal>,
    pub body_view: Option<ComponentSignal>,
    pub footer: Option<ComponentSignal>,
}

impl CardProps {
    pub fn new() -> Self {
        CardProps {
            ..Default::default()
        }
    }

    #[inline]
    #[must_use]
    pub fn with_title<A: Into<String>>(mut self, title: A, sub_title: Option<A>) -> Self {
        self.header_view = Some(
            html!("div", {
                .children(
                    vec![
                        Some(html!("div", { .class("title").text(title.into().as_str()) })),
                        sub_title.map(
                            |sub| html!("div", { .class("sub-title") .text(sub.into().as_str()) }),
                        ),
                    ].into_iter()
                    .filter_map(|d| d)
                )
            })
            .into(),
        );

        self
    }

    #[inline]
    #[must_use]
    pub fn body<T: Into<ComponentSignal>>(mut self, body: T) -> Self {
        self.body_view = Some(body.into());
        self
    }

    #[inline]
    #[must_use]
    pub fn header<T: Into<ComponentSignal>>(mut self, header: T) -> Self {
        self.header_view = Some(header.into());
        self
    }

    #[inline]
    #[must_use]
    pub fn header_signal<T: Signal<Item = U> + Unpin + 'static, U>(mut self, header: T) -> Self
    where
        U: Into<DomOption>,
    {
        self.header_view = Some(ComponentSignal::from_signal(header));
        self
    }

    #[inline]
    #[must_use]
    pub fn footer<T: Into<ComponentSignal>>(mut self, footer: T) -> Self {
        self.footer = Some(footer.into());
        self
    }
}

pub fn card<F>(props: CardProps, mixin: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let head = props.header_view;
    let body = props.body_view;
    let footer = props.footer;

    let children = vec![
        html!("div", {
            .class("dmat-card-header-container")
            .apply_if(head.is_some(), move |dom| {
                dom.child_signal(head.unwrap().0)
            })
        }),
        html!("div", {
            .class("dmat-card-body-container")
            .apply_if(body.is_some(), move |dom| {
                dom.child_signal(body.unwrap().0)
            })
        }),
        html!("div", {
            .class("dmat-card-footer-container")
            .apply_if(footer.is_some(), move |dom| {
                dom.child_signal(footer.unwrap().0)
            })
        }),
    ];

    html!("div", {
        .class("dmat-card")
        .apply(mixin)
        .children(children.into_iter())
    })
}
