use dominator::{events, html, Dom, DomBuilder};
use futures_signals::signal::{Always, Signal};
use web_sys::HtmlElement;

use crate::components::mixins::disabled_signal_mixin;

#[derive(Default)]
pub enum ButtonType {
    Elevated,
    #[default]
    Contained,
    Outlined,
    Text,
}

pub enum ButtonStyle {
    Prominent,
    Neutral,
    Unimportant,
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
pub struct ButtonProps<
    FClickCallback: Fn(events::Click) = fn(events::Click) -> (),
    TDisabledSignal: Signal<Item = bool> + Unpin = Always<bool>,
> {
    pub content: Option<ButtonContent>,
    pub click_handler: Option<FClickCallback>,
    pub button_type: ButtonType,
    pub style: ButtonStyle,
    pub disabled_signal: Option<TDisabledSignal>,
    pub apply: Option<Box<dyn FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
}

impl ButtonProps {
    pub fn new() -> ButtonProps {
        Self {
            content: None,
            click_handler: None,
            button_type: ButtonType::Contained,
            style: ButtonStyle::Prominent,
            disabled_signal: None,
            apply: None,
        }
    }
}

impl<FClickCallback: Fn(events::Click), TDisabledSignal: Signal<Item = bool> + Unpin>
    ButtonProps<FClickCallback, TDisabledSignal>
{
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
    pub fn label<U>(mut self, label: U) -> Self
    where
        U: ToString,
    {
        self.content = Some(ButtonContent::Label(label.to_string()));
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
    pub fn disabled_signal<TDisabledSignalNew: Signal<Item = bool> + Unpin>(
        self,
        disabled_signal: TDisabledSignalNew,
    ) -> ButtonProps<FClickCallback, TDisabledSignalNew> {
        ButtonProps {
            content: self.content,
            click_handler: self.click_handler,
            button_type: self.button_type,
            style: self.style,
            disabled_signal: Some(disabled_signal),
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn click_handler<FClickCallbackNew: Fn(events::Click) + 'static>(
        self,
        click_handler: FClickCallbackNew,
    ) -> ButtonProps<FClickCallbackNew, TDisabledSignal> {
        ButtonProps {
            content: self.content,
            click_handler: Some(click_handler),
            button_type: self.button_type,
            style: self.style,
            disabled_signal: self.disabled_signal,
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn apply<F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static>(
        mut self,
        apply: F,
    ) -> Self {
        self.apply = Some(Box::new(apply));
        self
    }
}

#[macro_export]
macro_rules! button {
    ($($methods:tt)*) => {{
        let default_props = $crate::components::button::ButtonProps::new();
        let applied_props = dominator::apply_methods!(default_props, $($methods)*);
        $crate::components::button::button(applied_props)
    }};
}

#[inline]
pub fn button<FClickCallback, TDisabledSignal>(
    button_props: ButtonProps<FClickCallback, TDisabledSignal>,
) -> Dom
where
    FClickCallback: Fn(events::Click) + 'static,
    TDisabledSignal: Signal<Item = bool> + Unpin + 'static,
{
    let content = button_props.content;
    let click_handler = button_props.click_handler;
    let disabled_signal = button_props.disabled_signal;
    let mixin = button_props.apply;

    html!("button", {
        .class("dmat-button")
        .apply_if(mixin.is_some(), |b| b.apply(mixin.unwrap()))
        .class( match button_props.button_type {
            ButtonType::Contained => "-contained",
            ButtonType::Outlined => "-outlined",
            ButtonType::Text => "-text",
            ButtonType::Elevated => "-elevated",
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
        .apply_if(click_handler.is_some(), |b| b.event(click_handler.unwrap()))
        .apply_if(disabled_signal.is_some(), |b| b.apply(disabled_signal_mixin(disabled_signal.unwrap())))
    })
}

#[cfg(test)]
mod test {
    use dominator::events::Click;
    use dominator::{clone, html};
    use futures_signals::signal::{Mutable, SignalExt};
    use wasm_bindgen_test::*;
    use web_sys::{HtmlButtonElement, HtmlElement};

    use dominator_testing::{async_yield, mount_test_dom, test_dyn_element_by_id};

    #[wasm_bindgen_test]
    async fn button_test() {
        let counter = Mutable::new(0);

        let btn = button!({
            .click_handler( clone!(counter => move |_: Click| {
                    counter.set(counter.get() + 1)
                }))
            .content(html!("span"))
            .disabled_signal(counter.signal_cloned().map(|v| v > 0))
            .apply(|dom| dom.attr("id", "test-button"))
        });

        mount_test_dom(btn);

        test_dyn_element_by_id("test-button", |ele: &HtmlElement| {
            ele.click();
        });

        assert_eq!(counter.get(), 1);

        // We need to yield to v8 so that the disabled property actually propagates here :/
        async_yield().await;

        // Verify the counter won't increment after disabling the button
        test_dyn_element_by_id("test-button", |ele: &HtmlElement| {
            ele.click();
        });

        assert_eq!(counter.get(), 1);

        async_yield().await;

        test_dyn_element_by_id("test-button", |ele: &HtmlButtonElement| {
            assert!(ele.disabled());
        });
    }
}
