use dominator::{html, Dom};
use futures_signals::signal::SignalExt;
use futures_signals_utils::split_signal::split_signal;

use crate::components::input::label::label_element;
use crate::components::input::validation_result::ValidationResult;

use crate::components::mixins::{assistive_text, disabled_signal_mixin};

#[component(render_fn = input_wrapper)]
struct InputWrapper {
    input: Dom,

    #[signal]
    #[default(None)]
    label: Option<Dom>,

    #[signal]
    #[default("".to_string())]
    value: String,

    #[signal]
    #[default(ValidationResult::Valid)]
    is_valid: ValidationResult,

    #[signal]
    #[default(false)]
    disabled: bool,

    #[signal]
    #[default(None)]
    assistive_text: Option<Dom>,

    #[signal]
    #[default(false)]
    has_focus: bool,

    class_name: String,

    extra_child: Dom,

    #[default(None)]
    input_id: Option<String>,
}

pub fn input_wrapper(props: impl InputWrapperPropsTrait + 'static) -> Dom {
    let InputWrapperProps {
        input,
        label,
        value,
        is_valid,
        disabled: disabled_signal,
        assistive_text: assistive_text_signal,
        has_focus,
        class_name,
        extra_child,
        input_id,
        apply,
    } = props.take();

    let label_element = label_element(value.map(|v| !v.is_empty()), has_focus, label, input_id);

    let (assistive_text_signal, has_assistive) =
        split_signal(assistive_text_signal, false, |v| v.is_some());

    let assistive = assistive_text(assistive_text_signal);
    let (error, has_error) = split_signal(is_valid, false, |v| !v.is_valid());
    let error = error.map(|e| match e {
        ValidationResult::Invalid { message } => Some(html!("div", {
            .class("error")
            .text(message.as_str())
        })),
        _ => None,
    });

    html!("div", {
        .apply_if(apply.is_some(), |d| d.apply(apply.unwrap()))
        .class_signal("-with-assistive",has_assistive)
        .class_signal("-invalid", has_error)
        .apply(disabled_signal_mixin(disabled_signal))
        .apply_if(class_name.is_some(), |d| d.class(class_name.unwrap()))
        .child(label_element)
        .apply_if(input.is_some(), |d| d.child(input.unwrap()))
        .apply_if(extra_child.is_some(), |d| d.child(extra_child.unwrap()))
        .child(html!("div", {
            .class("supporting-text")
            .child_signal(assistive)
            .child_signal(error)
        }))
    })
}
