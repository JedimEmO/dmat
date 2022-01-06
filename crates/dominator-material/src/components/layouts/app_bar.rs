use crate::utils::component_signal::{ComponentSignal, DomOption};
use dominator::{html, Dom, DomBuilder};
use futures_signals::signal::Signal;
use web_sys::HtmlElement;

#[derive(Clone)]
pub enum AppBarType {
    Normal,
    Prominent,
}

impl Default for AppBarType {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Default)]
pub struct AppBarProps {
    main_view: Option<ComponentSignal>,
    header_view: Option<ComponentSignal>,
    app_bar_type: AppBarType,
    fixed: bool,
}

impl AppBarProps {
    pub fn new() -> Self {
        Self {
            main_view: None,
            header_view: None,
            app_bar_type: AppBarType::Normal,
            fixed: false,
        }
    }

    #[inline]
    #[must_use]
    pub fn bar_type(mut self, bar_type: AppBarType) -> Self {
        self.app_bar_type = bar_type;
        self
    }

    #[inline]
    #[must_use]
    pub fn fixed(mut self) -> Self {
        self.fixed = true;
        self
    }

    #[inline]
    #[must_use]
    pub fn header<T: Into<ComponentSignal>>(mut self, child: T) -> Self {
        self.header_view = Some(child.into());
        self
    }

    #[inline]
    #[must_use]
    pub fn main<T: Into<ComponentSignal>>(mut self, child: T) -> Self {
        self.main_view = Some(child.into());
        self
    }

    #[inline]
    #[must_use]
    pub fn main_signal<T: Signal<Item = U> + Unpin + 'static, U>(mut self, child: T) -> Self
    where
        U: Into<DomOption>,
    {
        self.main_view = Some(ComponentSignal::from_signal(child));
        self
    }
}

#[inline]
pub fn app_bar<F>(props: AppBarProps, mixin: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let type_class = match props.app_bar_type {
        AppBarType::Normal => "-normal",
        AppBarType::Prominent => "-prominent",
    };

    let main_view = props.main_view;
    let header_view = props.header_view;

    html!("div", {
        .class("dmat-app-bar")
        .apply(mixin)
        .apply_if(props.fixed, move |dom| dom.class("-fixed"))
        .child(html!("div", {
            .class("viewport")
            .children(&mut [
                html!("div", {
                    .apply_if(main_view.is_some(), move |main| {
                        main.class("main")
                        .class(type_class)
                        .child_signal(main_view.unwrap().0)
                    })
                }),
                html!("div", {
                    .apply_if(header_view.is_some(), move |header| {
                         header.class("header")
                        .child_signal(header_view.unwrap().0)
                    })
                })
            ])
        }))
    })
}
