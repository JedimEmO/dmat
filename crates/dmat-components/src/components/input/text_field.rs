use crate::components::input::input_field::{input_wrapper, InputWrapperProps};
use crate::components::input::validation_result::ValidationResult;
use crate::components::input::value_adapters::mutable_t_value_adapter::MutableTValueAdapter;
use crate::components::input::value_adapters::value_adapter::ValueAdapter;
use crate::components::mixins::disabled_signal_mixin;
use dominator::{clone, events, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal, SignalExt};

#[component(render_fn = text_field)]
pub struct TextField<
    TValueAdapter: ValueAdapter + 'static = MutableTValueAdapter<String>,
    TOnFocusChange: Fn(bool) = fn(bool) -> (),
> {
    #[signal]
    #[default(None)]
    label: Option<Dom>,

    #[default(MutableTValueAdapter::default())]
    value: TValueAdapter,

    #[signal]
    #[default(ValidationResult::Valid)]
    is_valid: ValidationResult,

    #[signal]
    #[default(false)]
    disabled: bool,

    #[signal]
    #[default(None)]
    assistive_text: Option<Dom>,

    #[default(false)]
    claim_focus: bool,

    #[default(None)]
    input_id: Option<String>,

    on_focus_change: TOnFocusChange,
}

/// Creates a text input element for accepting user data
///
/// The return tuple contains:
/// 0: input Dom entry
/// 1: output of the component, containing a boolean signal for the  validity of the input according to the validator
pub fn text_field(props: impl TextFieldPropsTrait + 'static) -> Dom {
    let TextFieldProps {
        label,
        value,
        is_valid,
        disabled,
        assistive_text,
        claim_focus,
        input_id,
        on_focus_change,
        apply,
    } = props.take();

    let value_signal = value.get_value_signal();
    let sanitize_result = Mutable::new(ValidationResult::Valid);
    let has_focus = Mutable::new(false);

    let input_element = text_field_input(
        value,
        &sanitize_result,
        disabled,
        has_focus.clone(),
        claim_focus,
        input_id.clone(),
        on_focus_change,
    );

    let is_valid_combined = map_ref! {
        let is_valid_outer = is_valid,
        let is_valid_sanitized = sanitize_result.signal_cloned()
            => {
                if !is_valid_outer.is_valid() {
                    is_valid_outer.clone()
                } else {
                    is_valid_sanitized.clone()
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

    input_wrapper(
        InputWrapperProps::new()
            .value_signal(value_combined_signal)
            .input(input_element)
            .has_focus_signal(has_focus.signal())
            .apply(|d| if let Some(a) = apply { a(d) } else { d })
            .class_name("dmat-input-text-field".to_string())
            .assistive_text_signal(assistive_text)
            .is_valid_signal(is_valid_combined)
            .label_signal(label)
            .input_id(input_id),
    )
}

#[inline]
fn text_field_input(
    value: impl ValueAdapter + 'static,
    sanitize_result: &Mutable<ValidationResult>,
    disabled_signal: impl Signal<Item = bool> + 'static,
    has_focus: Mutable<bool>,
    claim_focus: bool,
    input_id: Option<String>,
    on_focus_change: Option<impl Fn(bool) + 'static>,
) -> Dom {
    let value_signal = value.get_value_signal();
    let value_signal_reset = value.get_value_signal();

    // We let the external value signal override any sanitizing we do internally
    let reset_sanitize_result = value_signal_reset.for_each(clone!(sanitize_result => move |_| {
        sanitize_result.set(ValidationResult::Valid);
        async {}
    }));

    html!("input", {
        .apply_if(on_focus_change.is_some(), clone!(has_focus => move|d| {
            let on_focus_change = on_focus_change.unwrap();
            d.future(async move {
                has_focus.signal().for_each(|current_focus| {
                    on_focus_change(current_focus);
                    async {}
                }).await;
            })
        }))
        .apply_if(input_id.is_some(), clone!(input_id => move|builder| builder.attr("id", input_id.unwrap().as_str())))
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
        .apply(disabled_signal_mixin(disabled_signal))
    })
}

#[cfg(test)]
mod test {
    use futures_signals::signal::Mutable;
    use wasm_bindgen_test::*;

    use crate::components::input::validation_result::ValidationResult;
    use crate::components::input::value_adapters::mutable_t_value_adapter::MutableTValueAdapter;
    use crate::components::{text_field, TextFieldProps};

    #[wasm_bindgen_test]
    async fn text_field_validation() {
        let val = Mutable::new("".to_string());

        let field = text_field!({
            .value(MutableTValueAdapter::new_simple(&val))
            .is_valid_signal(val.signal_ref(|v| {
                if v == "hello" {
                    ValidationResult::Valid
                } else {
                    ValidationResult::Invalid { message: "not hello".to_string() }
                }
            }))
            .apply(|d| d.attr("id", "testfield"))
        });

        let field_dom = field;

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
