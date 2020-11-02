use crate::utils::renderable_child::RenderableChild;
use dominator::{clone, html, Dom, DomBuilder};
use web_sys::HtmlElement;

#[derive(Clone)]
pub enum AppBarType {
    Normal,
    Prominent,
}

pub struct AppBarFinal<Header, Main> {
    apply: Option<Box<dyn FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
    main_view_signal: Main,
    header_view: Header,
    app_bar_type: AppBarType,
    fixed: bool,
}

impl<Header: RenderableChild, Main: RenderableChild> AppBarFinal<Header, Main> {
    #[inline]
    pub fn bar_type(mut self, bar_type: AppBarType) -> Self {
        self.app_bar_type = bar_type;
        self
    }

    #[inline]
    pub fn fixed(mut self) -> Self {
        self.fixed = true;
        self
    }

    #[inline]
    pub fn apply<F: 'static>(mut self, apply: F) -> Self
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    {
        self.apply = Some(Box::new(apply));
        self
    }

    pub fn render(self) -> Dom {
        let mut apply = self.apply;
        let type_class = match self.app_bar_type {
            AppBarType::Normal => "-normal",
            AppBarType::Prominent => "-prominent",
        };

        html!("div", {
            .class("dmat-app-bar")
            .apply_if(apply.is_some(), move |dom| {
                dom.apply(apply.take().unwrap())
            })
            .apply_if(self.fixed, move |dom| dom.class("-fixed"))
            .child(html!("div", {
                .class("viewport")
                .children(&mut [
                    self.main_view_signal.render(clone!(type_class => move |v| {
                        v.class("main").class(type_class)
                    })),
                    self.header_view.render(clone!(type_class => move |v| {
                        v.class("header").class(type_class)
                    }))
                ])
            }))
        })
    }
}

// Builders

pub struct AppBar {
    _private: std::marker::PhantomData<bool>,
}

pub struct AppBarBuilder2<H: RenderableChild> {
    header: H,
}

impl AppBar {
    pub fn new() -> AppBar {
        AppBar {
            _private: Default::default(),
        }
    }

    pub fn header<C: RenderableChild + 'static>(self, child: C) -> AppBarBuilder2<C> {
        AppBarBuilder2 { header: child }
    }
}

impl<H: RenderableChild> AppBarBuilder2<H> {
    pub fn main<C: RenderableChild + 'static>(self, child: C) -> AppBarFinal<H, C> {
        AppBarFinal {
            apply: None,
            main_view_signal: child,
            header_view: self.header,
            app_bar_type: AppBarType::Normal,
            fixed: false,
        }
    }
}
