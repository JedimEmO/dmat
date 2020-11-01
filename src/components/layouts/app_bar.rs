use crate::utils::renderable_child::RenderableChild;
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::signal::{always, Always, Map, Signal, SignalExt};
use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::rc::Rc;
use web_sys::HtmlElement;

pub struct AppBarFinal<Header, Main> {
    apply: Option<Box<dyn FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
    main_view_signal: Main,
    header_view: Header,
}

impl<Header: RenderableChild, Main: RenderableChild> AppBarFinal<Header, Main> {
    #[inline]
    pub fn apply<F: 'static>(mut self, apply: F) -> Self
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    {
        self.apply = Some(Box::new(apply));
        self
    }

    pub fn render(mut self) -> Dom {
        let mut apply = self.apply;
        html!("div", {
            .class("dmat-app-bar")
            .apply_if(apply.is_some(), move |dom| {
                dom.apply(apply.take().unwrap())
            })
            .child(html!("div", {
                .class("viewport")
                .children(&mut [
                    self.main_view_signal.render(|v| v.class("main")),
                    self.header_view.render(|v| v.class("header"))
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
        }
    }
}
