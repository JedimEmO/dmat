use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::Signal;

use wasm_bindgen::__rt::std::rc::Rc;
use web_sys::HtmlElement;

pub enum ButtonType {
    Contained,
    Outlined,
    Text,
}

pub enum ButtonStyle {
    Prominent,
    Neutral,
    Unimportant,
}

impl Default for ButtonType {
    fn default() -> Self {
        ButtonType::Contained
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self::Prominent
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
    pub style: ButtonStyle,
    pub disabled_signal: Option<Box<dyn Signal<Item = bool> + Unpin>>,
}

impl ButtonProps {
    pub fn new() -> Self {
        Self {
            content: None,
            click_handler: None,
            button_type: ButtonType::Contained,
            style: ButtonStyle::Prominent,
            disabled_signal: None,
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

    #[inline]
    #[must_use]
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    #[inline]
    #[must_use]
    pub fn disabled_signal<TSig>(mut self, signal: TSig) -> Self
    where
        TSig: Signal<Item = bool> + Unpin + 'static,
    {
        self.disabled_signal = Some(Box::new(signal));
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
    let disabled_signal = button_props.disabled_signal;

    html!("button", {
        .class("dmat-button")
        .apply(mixin)
        .class( match button_props.button_type {
            ButtonType::Contained => "-contained",
            ButtonType::Outlined => "-outlined",
            ButtonType::Text => "-text",
        })
        .class(match button_props.style {
            ButtonStyle::Prominent => "-prominent",
            ButtonStyle::Neutral => "-neutral",
            ButtonStyle::Unimportant => "-unimportant",
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
        .apply_if(disabled_signal.is_some(), move |dom_builder| {
            dom_builder.attribute_signal("disabled", map_ref!(let is_disabled = disabled_signal.unwrap() => {
                match is_disabled {
                    true => Some("disabled"),
                    _ => None
                }
            }))
        })
    })
}

#[cfg(test)]
mod test {
    use crate::components::ButtonProps;
    use dominator::events::Click;
    use dominator::{clone, html};
    use dominator_testing::{mount_test_dom, test_dyn_element_by_id};
    use futures_signals::signal::Mutable;
    use wasm_bindgen_test::*;
    use web_sys::HtmlElement;

    #[wasm_bindgen_test]
    fn button_test() {
        let counter = Mutable::new(0);

        let btn = button!(
            ButtonProps::new()
                .content(html!("span"))
                .on_click(clone!(counter => move |_: Click| {
                    counter.set(counter.get() + 1)
                })),
            |d| d.attribute("id", "test-button")
        );

        mount_test_dom(btn);

        test_dyn_element_by_id("test-button", |ele: &HtmlElement| {
            ele.click();
        });

        assert_eq!(counter.get(), 1);

        test_dyn_element_by_id("test-button", |ele: &HtmlElement| {
            ele.click();
        });

        assert_eq!(counter.get(), 2);
    }
}
