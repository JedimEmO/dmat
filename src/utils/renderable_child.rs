use dominator::{html, Dom, DomBuilder};
use futures_signals::signal::Signal;
use web_sys::HtmlElement;

pub trait RenderableChild {
    fn render<F>(self, apply: F) -> Dom
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>;
}

pub struct RenderableDom(Dom);

pub trait IntoRenderableChild {
    fn into_renderable_child(self) -> RenderableDom;
}

impl IntoRenderableChild for Dom {
    fn into_renderable_child(self) -> RenderableDom {
        RenderableDom(self)
    }
}

impl RenderableChild for RenderableDom {
    fn render<F>(self, apply: F) -> Dom
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    {
        html!("div", {
            .apply(apply)
            .child(self.0)
        })
    }
}

impl<T: 'static> RenderableChild for T
where
    T: Signal<Item = Option<Dom>> + Unpin,
{
    fn render<F>(self, apply: F) -> Dom
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    {
        html!("div", {
            .apply(apply)
            .child_signal(self)
        })
    }
}
