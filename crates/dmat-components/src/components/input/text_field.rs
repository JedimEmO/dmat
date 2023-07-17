use crate::components::input::input_field::{input_wrapper, InputWrapperProps};
use crate::components::input::validation_result::ValidationResult;
use crate::components::input::value_adapters::mutable_t_value_adapter::MutableTValueAdapter;
use crate::components::input::value_adapters::value_adapter::ValueAdapter;
use dominator::{clone, events, html, Dom};
use futures_signals::signal::{Mutable, MutableSignalCloned, SignalExt};
use futures_signals::{map_mut, map_ref};

#[component(render_fn = text_field)]
pub struct TextField<TValueAdapter: ValueAdapter + 'static = MutableTValueAdapter<String>> {
    #[signal]
    #[default(None)]
    label: Option<Dom>,

    #[default(MutableTValueAdapter::default())]
    value: TValueAdapter,

    #[signal]
    #[default(true)]
    is_valid: bool,

    #[signal]
    #[default(false)]
    disabled: bool,

    #[signal]
    #[default(None)]
    assistive_text: Option<Dom>,

    #[signal]
    #[default(None)]
    error_text: Option<Dom>,

    #[default(false)]
    claim_focus: bool,
}

pub struct TextFieldOutput {
    pub has_focus: MutableSignalCloned<bool>,
}

/// Creates a text input element for accepting user data
///
/// The return tuple contains:
/// 0: input Dom entry
/// 1: output of the component, containing a boolean signal for the  validity of the input according to the validator
pub fn text_field(props: impl TextFieldPropsTrait + 'static) -> (Dom, TextFieldOutput) {
    let TextFieldProps {
        label,
        value,
        is_valid,
        disabled,
        assistive_text,
        error_text,
        claim_focus,
        apply,
    } = props.take();

    let value_signal = value.get_value_signal();
    let sanitize_result = Mutable::new(ValidationResult::Valid);
    let has_focus = Mutable::new(false);

    let input_element = text_field_input(value, &sanitize_result, has_focus.clone(), claim_focus);

    let is_valid_combined = map_ref! {
        let is_valid_outer = is_valid,
        let is_valid_sanitized = sanitize_result.signal_cloned()
            => {
                *is_valid_outer && is_valid_sanitized.is_valid()
            }
    };

    let error_text_combined = map_mut! {
        let error_text_outer = error_text,
        let error_text_sanitized = sanitize_result.signal_cloned()
            => {
                if error_text_outer.is_some() {
                    error_text_outer.take()
                } else if let ValidationResult::Invalid { message } = error_text_sanitized {
                    Some(html!("div", {
                        .text(message)
                    }))
                } else {
                    None
                }
            }
    };

    let value_combined_signal = map_ref! {
        let value_outer = value_signal,
        let value_sanitized = sanitize_result.signal_cloned()
            => {
                if let ValidationResult::Valid = value_sanitized {
                    value_outer.clone()
                } else {
                    // We don't care about the contents of the value if it is invalid
                    "whatever".to_string()
                }
            }
    };

    (
        input_wrapper(
            InputWrapperProps::new()
                .value_signal(value_combined_signal)
                .input(input_element)
                .has_focus_signal(has_focus.signal())
                .apply(|d| if let Some(a) = apply { a(d) } else { d })
                .has_focus_signal(has_focus.signal())
                .class_name("dmat-input-text-field".to_string())
                .error_text_signal(error_text_combined)
                .assistive_text_signal(assistive_text)
                .disabled_signal(disabled)
                .is_valid_signal(is_valid_combined)
                .label_signal(label),
        ),
        TextFieldOutput {
            has_focus: has_focus.signal_cloned(),
        },
    )
}

#[inline]
fn text_field_input(
    value: impl ValueAdapter + 'static,
    sanitize_result: &Mutable<ValidationResult>,
    has_focus: Mutable<bool>,
    claim_focus: bool,
) -> Dom {
    let value_signal = value.get_value_signal();
    let value_signal_reset = value.get_value_signal();

    // We let the external value signal override any sanitizing we do internally
    let reset_sanitize_result = value_signal_reset.for_each(clone!(sanitize_result => move |_| {
        sanitize_result.set(ValidationResult::Valid);
        async {}
    }));

    html!("input", {
        .apply_if(claim_focus, clone!(has_focus => move|builder| {
            has_focus.set(true);
            builder.focused(true)
        }))
        .future(reset_sanitize_result)
        .event(clone!(sanitize_result => move |e: events::Input| {
            #[allow(deprecated)]
            if let Some(val) = e.value() {
                if let ValidationResult::Invalid { message } = value.set_value(val) {
                    sanitize_result.set(ValidationResult::Invalid { message })
                } else {
                    sanitize_result.set(ValidationResult::Valid)
                }
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
        .prop_signal("value", value_signal)
        .class("dmat-input-element")
    })
}

#[cfg(test)]
mod test {
    use futures_signals::signal::Mutable;
    use wasm_bindgen_test::*;

    use crate::components::input::value_adapters::mutable_t_value_adapter::MutableTValueAdapter;
    use crate::components::{text_field, TextFieldProps};

    #[wasm_bindgen_test]
    async fn text_field_validation() {
        let val = Mutable::new("".to_string());

        let field = text_field!({
            .value(MutableTValueAdapter::new_simple(&val))
            .is_valid_signal(val.signal_ref(|v| v == "hello"))
            .apply(|d| d.attr("id", "testfield"))
        });

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
