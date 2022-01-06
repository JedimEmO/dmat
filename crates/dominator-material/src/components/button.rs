use dominator::{clone, events, html, Dom, DomBuilder};

use wasm_bindgen::__rt::std::rc::Rc;
use web_sys::HtmlElement;

pub enum ButtonType {
    Contained,
    Outlined,
    Text,
}

impl Default for ButtonType {
    fn default() -> Self {
        ButtonType::Contained
    }
}

pub enum ButtonContent {
    Label(String),
    Dom(Dom),
}

#[derive(Default)]
pub struct ButtonProps {
    pub content: Option<ButtonContent>,
    pub click_handler: Option<Rc<dyn Fn(events::Click)>>,
    pub button_type: ButtonType,
}

impl ButtonProps {
    pub fn new() -> Self {
        Self {
            content: None,
            click_handler: None,
            button_type: ButtonType::Contained,
        }
    }

    #[inline]
    #[must_use]
    pub fn content<U>(mut self, content: U) -> Self
    where
        U: Into<Dom>,
    {
        self.content = Some(ButtonContent::Dom(content.into()));
        self
    }

    #[inline]
    #[must_use]
    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(events::Click) + 'static,
    {
        self.click_handler = Some(Rc::new(handler));
        self
    }

    #[inline]
    #[must_use]
    pub fn button_type(mut self, button_type: ButtonType) -> Self {
        self.button_type = button_type;
        self
    }
}

#[macro_export]
macro_rules! button {
    ($props: expr) => {{
        $crate::components::button::button($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::button::button($props, $mixin)
    }};
}

#[inline]
pub fn button<F>(button_props: ButtonProps, mixin: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let content = button_props.content;
    let click_handler = button_props.click_handler.clone();

    html!("button", {
        .class("dmat-button")
        .apply(mixin)
        .class( match button_props.button_type {
            ButtonType::Contained => "-contained",
            ButtonType::Outlined => "-outlined",
            ButtonType::Text => "-text",
        })
        .apply(move |bdom| {
            match content {
                Some(ButtonContent::Label(label)) => bdom.text(label.as_str()),
                Some(ButtonContent::Dom(dom)) => bdom.child(dom),
                _ => bdom
            }
        })
        .apply_if(button_props.click_handler.is_some(), |dom| {
            dom.event(clone!(click_handler => move |e: events::Click| {
                if let Some(handler) = &click_handler {
                    (&handler.as_ref())(e);
                }
            }))
        })
    })
}
