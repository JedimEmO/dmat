use dominator::{clone, events, html, Dom};
use futures_signals::signal::{Always, Mutable, MutableSignalCloned, Signal};

use crate::components::input::input_field::input;
use crate::components::input::input_props::{InputProps};
use crate::utils::mixin::ApplyMixin;

pub struct TextFieldProps<
    TLabelSignal: Signal<Item=Option<String>> + Unpin + 'static = Always<Option<String>>,
    TValidSignal: Signal<Item=bool> + Unpin + 'static = Always<bool>,
    TAssistiveTextSignal: Signal<Item=Option<String>> + Unpin + 'static = Always<Option<String>>,
    TErrorTextSignal: Signal<Item=Option<String>> + Unpin + 'static = Always<Option<String>>,
    TDisabledSignal: Signal<Item=bool> + Unpin + 'static = Always<bool>,
> {
    pub claim_focus: bool,
    pub input_props: InputProps<
        TLabelSignal,
        TValidSignal,
        TAssistiveTextSignal,
        TErrorTextSignal,
        TDisabledSignal,
    >,
    pub apply: ApplyMixin,
}

impl<
    TLabelSignal: Signal<Item=Option<String>> + Unpin + 'static,
    TValidSignal: Signal<Item=bool> + Unpin + 'static,
    TAssistiveTextSignal: Signal<Item=Option<String>> + Unpin + 'static,
    TErrorTextSignal: Signal<Item=Option<String>> + Unpin + 'static,
    TDisabledSignal: Signal<Item=bool> + Unpin + 'static
>
TextFieldProps<
    TLabelSignal,
    TValidSignal,
    TAssistiveTextSignal,
    TErrorTextSignal,
    TDisabledSignal,
>
{
    #[inline]
    #[must_use]
    pub fn claim_focus(mut self) -> Self {
        self.claim_focus = true;
        self
    }

    #[inline]
    #[must_use]
    pub fn apply(mut self, apply: ApplyMixin) -> Self {
        self.apply = apply;
        self
    }

    #[inline]
    #[must_use]
    pub fn is_valid(
        self,
        is_valid: bool,
    ) -> TextFieldProps<TLabelSignal,
        Always<bool>,
        TAssistiveTextSignal,
        TErrorTextSignal,
        TDisabledSignal, > {
        TextFieldProps {
            claim_focus: self.claim_focus,
            input_props: self.input_props.is_valid(is_valid),
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn is_valid_signal<TNewValidSignal: Signal<Item=bool> + Unpin + 'static>(
        self,
        is_valid: TNewValidSignal,
    ) -> TextFieldProps<TLabelSignal,
        TNewValidSignal,
        TAssistiveTextSignal,
        TErrorTextSignal,
        TDisabledSignal, > {
        TextFieldProps {
            claim_focus: self.claim_focus,
            input_props: self.input_props.is_valid_signal(is_valid),
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn label<T: AsRef<str>>(
        self,
        label: T,
    ) -> TextFieldProps<Always<Option<String>>,
        TValidSignal,
        TAssistiveTextSignal,
        TErrorTextSignal,
        TDisabledSignal, > {
        TextFieldProps {
            claim_focus: self.claim_focus,
            input_props: self.input_props.label(label),
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn label_signal(
        self,
        label: TLabelSignal,
    ) -> TextFieldProps<TLabelSignal,
        TValidSignal,
        TAssistiveTextSignal,
        TErrorTextSignal,
        TDisabledSignal, > {
        TextFieldProps {
            claim_focus: self.claim_focus,
            input_props: self.input_props.label_signal(label),
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn value(
        mut self,
        value: Mutable<String>,
    ) -> Self {
        self.input_props.value = value;

        self
    }

    #[inline]
    #[must_use]
    pub fn assistive_text(
        self,
        assistive_text: Option<String>,
    ) -> TextFieldProps<TLabelSignal,
        TValidSignal,
        Always<Option<String>>,
        TErrorTextSignal,
        TDisabledSignal, > {
        TextFieldProps {
            claim_focus: self.claim_focus,
            input_props: self.input_props.assistive_text(assistive_text),
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn assistive_text_signal<TNewAssistiveTextSignal: Signal<Item=Option<String>> + Unpin + 'static>(
        self,
        assistive_text: TNewAssistiveTextSignal,
    ) -> TextFieldProps<TLabelSignal,
        TValidSignal,
        TNewAssistiveTextSignal,
        TErrorTextSignal,
        TDisabledSignal, > {
        TextFieldProps {
            claim_focus: self.claim_focus,
            input_props: self.input_props.assistive_text_signal(assistive_text),
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn error_text(
        self,
        error_text: Option<String>,
    ) -> TextFieldProps<TLabelSignal,
        TValidSignal,
        TAssistiveTextSignal,
        Always<Option<String>>,
        TDisabledSignal, > {
        TextFieldProps {
            claim_focus: self.claim_focus,
            input_props: self.input_props.error_text(error_text),
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn error_text_signal<TNewErrorTextSignal: Signal<Item=Option<String>> + Unpin + 'static>(
        self,
        error_text: TNewErrorTextSignal,
    ) -> TextFieldProps<TLabelSignal,
        TValidSignal,
        TAssistiveTextSignal,
        TNewErrorTextSignal,
        TDisabledSignal, > {
        TextFieldProps {
            claim_focus: self.claim_focus,
            input_props: self.input_props.error_text_signal(error_text),
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn disabled(
        self
    ) -> TextFieldProps<TLabelSignal,
        TValidSignal,
        TAssistiveTextSignal,
        TErrorTextSignal,
        Always<bool>, > {
        TextFieldProps {
            claim_focus: self.claim_focus,
            input_props: self.input_props.disabled(true),
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn disabled_signal<TNewDisabledSignal: Signal<Item=bool> + Unpin + 'static>(
        self,
        disabled: TNewDisabledSignal,
    ) -> TextFieldProps<TLabelSignal,
        TValidSignal,
        TAssistiveTextSignal,
        TErrorTextSignal,
        TNewDisabledSignal, > {
        TextFieldProps {
            claim_focus: self.claim_focus,
            input_props: self.input_props.disabled_signal(disabled),
            apply: self.apply,
        }
    }
}

impl TextFieldProps {
    pub fn new() -> TextFieldProps {
        TextFieldProps {
            claim_focus: false,
            input_props: InputProps {
                label: None,
                value: Default::default(),
                is_valid: None,
                assistive_text_signal: None,
                error_text_signal: None,
                disabled_signal: None,
            },
            apply: Some(Box::new(|dom| dom)),
        }
    }
}

impl Default for TextFieldProps {
    fn default() -> Self {
        TextFieldProps::new()
    }
}

pub struct TextFieldOutput {
    pub has_focus: MutableSignalCloned<bool>,
}

#[macro_export]
macro_rules! text_field {
    ($($methods:tt)*) => {{
        let default_props = $crate::components::input::text_field::TextFieldProps::new();
        let applied_props = dominator::apply_methods!(default_props, $($methods)*);
        $crate::components::input::text_field::text_field(applied_props)
    }};
}

/// Creates a text input element for accepting user data
///
/// The return tuple contains:
/// 0: input Dom entry
/// 1: output of the component, containing a boolean signal for the  validity of the input according to the validator
pub fn text_field<
    TLabelSignal: Signal<Item=Option<String>> + Unpin + 'static,
    TValidSignal: Signal<Item=bool> + Unpin + 'static,
    TAssistiveTextSignal: Signal<Item=Option<String>> + Unpin + 'static,
    TErrorTextSignal: Signal<Item=Option<String>> + Unpin + 'static,
    TDisabledSignal: Signal<Item=bool> + Unpin + 'static
>(
    props: TextFieldProps<
        TLabelSignal,
        TValidSignal,
        TAssistiveTextSignal,
        TErrorTextSignal,
        TDisabledSignal,
    >
) -> (Dom, TextFieldOutput)
{
    let has_focus = Mutable::new(false);

    let input_element = text_field_input(&props.input_props.value, &has_focus, props.claim_focus);

    let mixin = props.apply;
    (
        input(
            input_element,
            &has_focus,
            props.input_props,
            |d| {
                if mixin.is_some() {
                    (mixin.unwrap())(d)
                } else {
                    d
                }
            },
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
                    is_valid: Some(val.signal_ref(|v| v == "hello")),
                    label: Some(always(None)),
                    assistive_text_signal: Some(always(None)),
                    error_text_signal: Some(always(None)),
                    disabled_signal: Some(always(false)),
                },
                apply: Some(Box::new(|d| d.attr("id", "testfield"))),
            }
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
