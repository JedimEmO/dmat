use crate::components::text;
use dominator::traits::AsStr;
use dominator::{clone, events, html, Dom};
use futures_signals::signal::{always, Signal, SignalExt};
use std::iter::{once, Once};
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

pub struct ButtonProps<TContentSignal: Signal<Item = Option<Dom>> + 'static> {
    pub content_signal: Once<TContentSignal>,
    pub click_handler: Option<Rc<dyn Fn(events::Click)>>,
    pub button_type: ButtonType,
}

impl<TContentSignal: Signal<Item = Option<Dom>> + 'static> ButtonProps<TContentSignal> {
    pub fn new(content_signal: Once<TContentSignal>) -> Self {
        Self {
            content_signal,
            click_handler: None,
            button_type: ButtonType::Contained,
        }
    }

    #[inline]
    pub fn on_click<F>(mut self: Self, handler: F) -> Self
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

pub fn button<TContentSignal: Signal<Item = Option<Dom>> + 'static>(
    props: ButtonProps<TContentSignal>,
) -> Dom {
    Dom::with_state(props, |button_props| {
        let click_handler = button_props.click_handler.clone();

        html!("button", {
            .class("dmat-button")
            .class( match button_props.button_type {
                ButtonType::Contained => "-contained",
                ButtonType::Outlined => "-outlined",
                ButtonType::Text => "-text",
            })
            .child_signal(button_props.content_signal.next().unwrap())
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
