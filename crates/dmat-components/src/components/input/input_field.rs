use dominator::{html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Broadcaster, Mutable, SignalExt};
use futures_signals_utils::split_signal::split_signal;

use crate::components::input::label::label_element;

use crate::components::mixins::{assistive_text, disabled_signal_mixin, invalid_signal_mixin};

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

    #[signal]
    #[default(false)]
    has_focus: bool,

    class_name: String,
    extra_child: Dom,
}

pub fn input_wrapper(props: impl InputWrapperPropsTrait + 'static) -> Dom {
    let InputWrapperProps {
        input,
        label,
        value,
        is_valid,
        disabled: disabled_signal,
        assistive_text: assistive_text_signal,
        error_text: error_text_signal,
        has_focus,
        class_name,
        extra_child,
        apply,
    } = props.take();

    let label_element = label_element(value.map(|v| !v.is_empty()), has_focus, label);

    let _has_error = Mutable::new(false);

    let is_valid_broadcast = Broadcaster::new(is_valid);

    let error_text_signal = error_text_signal.map(|v| {
        v.map(|v| {
            html!("div", {
                .class("-error-text")
                .child(v)
            })
        })
    });

    let (error_text_signal, has_error) = split_signal(error_text_signal, false, |v| v.is_some());
    let (assistive_text_signal, has_assistive) =
        split_signal(assistive_text_signal, false, |v| v.is_some());
    let assistive = assistive_text(assistive_text_signal);

    html!("div", {
        .apply_if(apply.is_some(), |d| d.apply(apply.unwrap()))
        .class_signal(
            "-with-assistive",
            map_ref!(
                let assistive = has_assistive,
                let err = has_error,
                let is_valid = is_valid_broadcast.signal() => {
                    *assistive || (*err && !*is_valid)
                }
            )
        )
        .class_signal(
            "-error",
            is_valid_broadcast.signal().map(|v| !v)
        )
        .apply(disabled_signal_mixin(disabled_signal))
        .apply(invalid_signal_mixin(is_valid_broadcast.signal()))
        .apply_if(class_name.is_some(), |d| d.class(class_name.unwrap()))
        .child(label_element)
        .apply_if(input.is_some(), |d| d.child(input.unwrap()))
        .apply_if(extra_child.is_some(), |d| d.child(extra_child.unwrap()))
        .child_signal(error_text_signal)
        .child_signal(assistive)
    })
}
