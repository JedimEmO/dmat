use crate::utils::component_signal::{ComponentSignal, DomOption};
use dominator::{clone, events, html, Dom};
use futures_signals::signal::Signal;

use wasm_bindgen::__rt::std::rc::Rc;

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
    Signal(Option<Box<dyn Signal<Item = Dom>>>),
}

impl Default for ButtonContent {
    fn default() -> Self {
        ButtonContent::Label("".to_string())
    }
}

pub struct ButtonProps {
    pub content_signal: Option<ComponentSignal>,
    pub click_handler: Option<Rc<dyn Fn(events::Click)>>,
    pub button_type: ButtonType,
}

impl ButtonProps {
    pub fn new() -> Self {
        Self {
            content_signal: None,
            click_handler: None,
            button_type: ButtonType::Contained,
        }
    }

    #[inline]
    pub fn content<T: Into<ComponentSignal>>(mut self, content: T) -> Self {
        self.content_signal = Some(content.into());
        self
    }

    #[inline]
    pub fn content_signal<T: Signal<Item = U> + Unpin + 'static, U>(mut self, content: T) -> Self
    where
        U: Into<DomOption>,
    {
        self.content_signal = Some(ComponentSignal::from_signal(content));
        self
    }

    #[inline]
    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(events::Click) + 'static,
    {
        self.click_handler = Some(Rc::new(handler));
        self
    }

    #[inline]
    pub fn button_type(mut self, button_type: ButtonType) -> Self {
        self.button_type = button_type;
        self
    }
}

pub fn button(mut props: ButtonProps) -> Dom {
    let content = props.content_signal.take();

    Dom::with_state(props, |button_props| {
        let click_handler = button_props.click_handler.clone();

        html!("button", {
            .class("dmat-button")
            .class( match button_props.button_type {
                ButtonType::Contained => "-contained",
                ButtonType::Outlined => "-outlined",
                ButtonType::Text => "-text",
            })
            .apply_if(content.is_some(), move |bdom| {
                bdom.child_signal(content.unwrap().0)
            })
            .apply_if(button_props.click_handler.is_some(), |dom| {
                dom.event(clone!(click_handler => move |e: events::Click| {
                    if let Some(handler) = &click_handler {
                        (&handler.as_ref())(e);
                    }
                }))
            })
        })
    })
}
