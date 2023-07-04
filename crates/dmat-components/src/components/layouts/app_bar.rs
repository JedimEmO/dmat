use dominator::{html, Dom, DomBuilder};
use futures_signals::signal::{always, Always};
use web_sys::HtmlElement;

use crate::utils::component_signal::{ComponentSignal, NoSignal};
use crate::utils::mixin::ApplyMixin;

#[inline]
pub fn app_bar<TMainSignal: ComponentSignal, THeaderSignal: ComponentSignal>(
    props: AppBarProps<TMainSignal, THeaderSignal>,
) -> Dom {
    let type_class = match props.app_bar_type {
        AppBarType::Normal => "-normal",
        AppBarType::Prominent => "-prominent",
    };

    let main_view = props.main_view;
    let header_view = props.header_view;
    let mixin = props.apply;

    html!("div", {
        .class("dmat-app-bar")
        .apply_if(mixin.is_some(), |dom| dom.apply(mixin.unwrap()))
        .apply_if(props.fixed, |dom| dom.class("-fixed"))
        .children(&mut [
            html!("header", {
                .apply_if(header_view.is_some(), move |header| {
                     header.class("header")
                    .child_signal(header_view.unwrap())
                })
            }),
            html!("main", {
                .apply_if(main_view.is_some(), move |main| {
                    main.class("main")
                    .class(type_class)
                    .child_signal(main_view.unwrap())
                })
            })
        ])
    })
}

#[macro_export]
macro_rules! app_bar {
    ($($methods:tt)*) => {{
        let default_props =$crate::components::layouts::app_bar::AppBarProps::new();
        let applied_props = dominator::apply_methods!(default_props, $($methods)*);
        $crate::components::layouts::app_bar::app_bar(applied_props)
    }};
}

#[derive(Clone, Default)]
pub enum AppBarType {
    #[default]
    Normal,
    Prominent,
}

pub struct AppBarProps<
    TMainSignal: ComponentSignal = NoSignal,
    THeaderSignal: ComponentSignal = NoSignal,
> {
    pub main_view: Option<TMainSignal>,
    pub header_view: Option<THeaderSignal>,
    pub app_bar_type: AppBarType,
    pub fixed: bool,
    pub apply: ApplyMixin
}

impl Default for AppBarProps {
    fn default() -> Self {
        Self::new()
    }
}

impl AppBarProps {
    pub fn new() -> AppBarProps {
        Self {
            main_view: None,
            header_view: None,
            app_bar_type: AppBarType::Normal,
            fixed: false,
            apply: None,
        }
    }
}

impl<TMainSignal: ComponentSignal, THeaderSignal: ComponentSignal>
    AppBarProps<TMainSignal, THeaderSignal>
{
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
    pub fn header(
        self,
        child: impl Into<Option<Dom>>,
    ) -> AppBarProps<TMainSignal, Always<Option<Dom>>> {
        AppBarProps {
            main_view: self.main_view,
            header_view: Some(always(child.into())),
            app_bar_type: self.app_bar_type,
            fixed: self.fixed,
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn header_signal<T: ComponentSignal>(self, child: T) -> AppBarProps<TMainSignal, T> {
        AppBarProps {
            main_view: self.main_view,
            header_view: Some(child),
            app_bar_type: self.app_bar_type,
            fixed: self.fixed,
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn main(
        self,
        child: impl Into<Option<Dom>>,
    ) -> AppBarProps<Always<Option<Dom>>, THeaderSignal> {
        AppBarProps {
            main_view: Some(always(child.into())),
            header_view: self.header_view,
            app_bar_type: self.app_bar_type,
            fixed: self.fixed,
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn main_signal<T: ComponentSignal>(self, child: T) -> AppBarProps<T, THeaderSignal> {
        AppBarProps {
            main_view: Some(child),
            header_view: self.header_view,
            app_bar_type: self.app_bar_type,
            fixed: self.fixed,
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn apply(
        mut self,
        apply: impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static,
    ) -> Self {
        self.apply = Some(Box::new(apply));
        self
    }
}
