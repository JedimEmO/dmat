use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::signal::{Mutable, MutableSignalCloned, Signal};
use web_sys::HtmlElement;

use crate::components::input::input_field::input;
use crate::components::input::input_props::InputProps;

pub struct TextFieldProps<
    TLabelSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TValidSignal: Signal<Item = bool> + Unpin + 'static,
    TAssistiveTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TErrorTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TDisabledSignal: Signal<Item = bool> + Unpin + 'static,
> {
    pub claim_focus: bool,
    pub input_props: InputProps<
        TLabelSignal,
        TValidSignal,
        TAssistiveTextSignal,
        TErrorTextSignal,
        TDisabledSignal,
    >,
}

impl<
        TLabelSignal: Signal<Item = Option<String>> + Unpin + 'static,
        TValidSignal: Signal<Item = bool> + Unpin + 'static,
        TAssistiveTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
        TErrorTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
        TDisabledSignal: Signal<Item = bool> + Unpin + 'static,
    >
    TextFieldProps<
        TLabelSignal,
        TValidSignal,
        TAssistiveTextSignal,
        TErrorTextSignal,
        TDisabledSignal,
    >
{
    pub fn new(
        value: Mutable<String>,
        label: TLabelSignal,
        is_valid: TValidSignal,
        assistive_text_signal: TAssistiveTextSignal,
        error_text_signal: TErrorTextSignal,
        disabled_signal: TDisabledSignal,
    ) -> Self {
        TextFieldProps {
            claim_focus: false,
            input_props: InputProps {
                label,
                value,
                is_valid,
                assistive_text_signal,
                error_text_signal,
                disabled_signal,
            },
        }
    }

    #[inline]
    #[must_use]
    pub fn claim_focus(mut self) -> Self {
        self.claim_focus = true;
        self
    }
}

pub struct TextFieldOutput {
    pub has_focus: MutableSignalCloned<bool>,
}

#[macro_export]
macro_rules! text_field {
    ($props: expr) => {{
        $crate::components::input::text_field::text_field($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::input::text_field::text_field($props, $mixin)
    }};
}

/// Creates a text input element for accepting user data
///
/// The return tuple contains:
/// 0: input Dom entry
/// 1: output of the component, containing a boolean signal for the  validity of the input according to the validator
pub fn text_field<
    TLabelSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TValidSignal: Signal<Item = bool> + Unpin + 'static,
    TAssistiveTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TErrorTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TDisabledSignal: Signal<Item = bool> + Unpin + 'static,
    F,
>(
    props: TextFieldProps<
        TLabelSignal,
        TValidSignal,
        TAssistiveTextSignal,
        TErrorTextSignal,
        TDisabledSignal,
    >,
    mixin: F,
) -> (Dom, TextFieldOutput)
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let has_focus = Mutable::new(false);

    let input_element = text_field_input(&props.input_props.value, &has_focus, props.claim_focus);

    (
        input(
            input_element,
            &has_focus,
            props.input_props,
            mixin,
            "dmat-input-text-field",
            None,
        ),
        TextFieldOutput {
            has_focus: has_focus.signal_cloned(),
        },
    )
}

#[inline]
fn text_field_input(value: &Mutable<String>, has_focus: &Mutable<bool>, claim_focus: bool) -> Dom {
    html!("input", {
        .apply_if(claim_focus, clone!(has_focus => move|builder| {
            has_focus.set(true);
            builder.focused(true)
        }))
        .event(clone!(value => move |e: events::Input| {
            #[allow(deprecated)]
            if let Some(val) = e.value() {
                value.replace(val);
            };

        }))
        .event(clone!(has_focus => {
            move |_e: events::Focus| {
                has_focus.set(true);
            }
        }))
        .event(clone!(has_focus => {
            move |_: events::Blur| {
                has_focus.set(false);
            }
        }))
        .prop_signal("value", value.signal_cloned())
        .class("dmat-input-element")
    })
}

#[cfg(test)]
mod test {
    use futures_signals::signal::{always, Mutable};
    use wasm_bindgen_test::*;

    use crate::components::input::input_props::InputProps;
    use crate::components::{text_field, TextFieldProps};

    #[wasm_bindgen_test]
    async fn text_field_validation() {
        let val = Mutable::new("".to_string());

        let field = text_field(
            TextFieldProps {
                claim_focus: false,
                input_props: InputProps {
                    value: val.clone(),
                    is_valid: val.signal_ref(|v| v == "hello"),
                    label: always(None),
                    assistive_text_signal: always(None),
                    error_text_signal: always(None),
                    disabled_signal: always(false),
                },
            },
            |d| d.attr("id", "testfield"),
        );

        let field_dom = field.0;
        let _field_out = field.1;

        dominator::append_dom(
            &web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap(),
            field_dom,
        );

        val.set("hello".to_string());

        // let mut valid_stream = field_out.is_valid.to_stream();
        //
        // while !valid_stream.next().await.unwrap() {}
    }
}
