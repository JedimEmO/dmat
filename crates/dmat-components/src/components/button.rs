use dominator::{events, html, Dom};
use futures_signals::signal::{always, SignalExt};

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

/// A button. You click it, things happen.
///
///
/// # Example
/// ```rust,no_run
/// use dmat_components::components::button::*;
/// use dominator::{clone, html, Dom};
/// use futures_signals::signal::Mutable;
///
/// fn counter() {
///     let counter_state = Mutable::new(0);
///     button!({
///         .on_click(clone!(counter_state => move |_| {
///             counter_state.set(counter_state.get() + 1);
///         }
///     })
/// }
/// ```
#[component(render_fn = button)]
pub struct Button<FClickCallback: Fn(events::Click) = fn(events::Click) -> ()> {
    #[signal]
    pub content: Dom,
    #[default(| _ | {})]
    pub click_handler: FClickCallback,
    #[default(ButtonType::default())]
    pub button_type: ButtonType,
    #[default(ButtonStyle::default())]
    pub style: ButtonStyle,
    #[signal]
    #[default(false)]
    pub disabled: bool,
}

#[inline]
pub fn button(button_props: impl ButtonPropsTrait + 'static) -> Dom {
    let ButtonProps {
        content,
        button_type,
        style,
        disabled,
        apply,
        click_handler,
    } = button_props.take();

    html!("button", {
        .class("dmat-button")
        .apply_if(apply.is_some(), |b| b.apply(apply.unwrap()))
        .class( match button_type {
            ButtonType::Contained => "-contained",
            ButtonType::Outlined => "-outlined",
            ButtonType::Text => "-text",
            ButtonType::Elevated => "-elevated",
        })
        .class(match style {
            ButtonStyle::Prominent => "-prominent",
            ButtonStyle::Neutral => "-neutral",
            ButtonStyle::Unimportant => "-unimportant",
        })
        .apply_if(content.is_some(), |d| d.child_signal(content.unwrap().map(Some)))
        .event(click_handler)
        .apply(disabled_signal_mixin(disabled))
    })
}

#[cfg(test)]
mod test {
    use dominator::{clone, events, html};
    use futures_signals::signal::Mutable;
    use futures_signals::signal::SignalExt;
    use wasm_bindgen_test::*;
    use web_sys::{HtmlButtonElement, HtmlElement};

    use crate::components::button::*;
    use dominator_testing::{async_yield, mount_test_dom, test_dyn_element_by_id};

    #[wasm_bindgen_test]
    async fn button_test() {
        let counter = Mutable::new(0);

        let btn = button!({
            .click_handler(clone!(counter => move |_: events::Click| {
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

/// Shorthand for creating a button with a label text
impl<
        TcontentSignal: futures_signals::signal::Signal<Item = Dom>,
        FClickCallback: Fn(events::Click),
        TdisabledSignal: futures_signals::signal::Signal<Item = bool>,
        TApplyFn: FnOnce(
            dominator::DomBuilder<web_sys::HtmlElement>,
        ) -> dominator::DomBuilder<web_sys::HtmlElement>,
    > ButtonProps<TcontentSignal, FClickCallback, TdisabledSignal, TApplyFn>
{
    pub fn label(
        self,
        v: impl AsRef<str>,
    ) -> ButtonProps<futures_signals::signal::Always<Dom>, FClickCallback, TdisabledSignal, TApplyFn>
    {
        ButtonProps {
            click_handler: self.click_handler,
            content: Some(always(html!("span", { .text(v.as_ref()) }))),
            button_type: self.button_type,
            style: self.style,
            disabled: self.disabled,
            apply: self.apply,
        }
    }
}
