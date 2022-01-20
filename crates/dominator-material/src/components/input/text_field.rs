use crate::components::input::input::input;
use crate::components::input::input_props::InputProps;
use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::signal::{always, Mutable, MutableSignalCloned, Signal};
use web_sys::HtmlElement;

pub struct TextFieldProps {
    pub claim_focus: bool,
    pub input_props: InputProps,
}

impl TextFieldProps {
    pub fn new(value: Mutable<String>) -> Self {
        TextFieldProps {
            claim_focus: false,
            input_props: InputProps {
                label: None,
                value,
                is_valid: None,
                assistive_text_signal: None,
                error_text_signal: None,
            },
        }
    }

    #[inline]
    #[must_use]
    pub fn label<TLabel: Into<String>>(mut self, label: TLabel) -> Self {
        self.input_props.label = Some(Box::new(always(label.into())));
        self
    }

    #[inline]
    #[must_use]
    pub fn label_signal<TLabel: Signal<Item = String> + Unpin + 'static>(
        mut self,
        label: TLabel,
    ) -> Self {
        self.input_props.label = Some(Box::new(label));
        self
    }

    #[inline]
    #[must_use]
    pub fn claim_focus(mut self) -> Self {
        self.claim_focus = true;
        self
    }

    #[inline]
    #[must_use]
    pub fn assistive_text_signal<TSig: Signal<Item = Option<String>> + Unpin + 'static>(
        mut self,
        sig: TSig,
    ) -> Self {
        self.input_props.assistive_text_signal = Some(Box::new(sig));
        self
    }

    #[inline]
    #[must_use]
    pub fn error_text_signal<TSig: Signal<Item = Option<String>> + Unpin + 'static>(
        mut self,
        sig: TSig,
    ) -> Self {
        self.input_props.error_text_signal = Some(Box::new(sig));
        self
    }

    #[inline]
    #[must_use]
    pub fn validator<TSig: Signal<Item = bool> + Unpin + 'static>(mut self, sig: TSig) -> Self {
        self.input_props.is_valid = Some(Box::new(sig));
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
pub fn text_field<F>(props: TextFieldProps, mixin: F) -> (Dom, TextFieldOutput)
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
        .property_signal("value", value.signal_cloned())
        .class("dmat-input-element")
    })
}

#[cfg(test)]
mod test {
    use futures_signals::signal::Mutable;
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
                    is_valid: Some(Box::new(val.signal_ref(|v| v == "hello"))),
                    label: None,
                    assistive_text_signal: None,
                    error_text_signal: None,
                },
            },
            |d| d.attribute("id", "testfield"),
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
