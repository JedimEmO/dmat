use dominator::{html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Broadcaster, Mutable, SignalExt};
use wasm_bindgen::UnwrapThrowExt;

use crate::components::input::label::label_element;
use crate::components::mixins::children_builder::build_children;
use crate::components::mixins::{
    assistive_text, disabled_signal_mixin, error_text, invalid_signal_mixin,
};

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

    let label_element = label_element(
        input.unwrap_throw(),
        value.map(|v| !v.is_empty()),
        has_focus,
        label,
    );
    let has_assistive = Mutable::new(false);
    let has_error = Mutable::new(false);

    let is_valid_broadcast = Broadcaster::new(is_valid);

    let error = Some(error_text(
        error_text_signal,
        is_valid_broadcast.signal(),
        &has_error,
    ));

    let assistive = assistive_text(assistive_text_signal, &has_assistive);

    let children = build_children(&mut [Some(label_element), extra_child, error, Some(assistive)]);

    html!("div", {
        .children(children.into_iter())
        .apply_if(apply.is_some(), |d| d.apply(apply.unwrap()))
        .class_signal(
            "assistive",
            map_ref!(
                let assistive = has_assistive.signal(),
                let err = has_error.signal() => {
                    *assistive || *err
                }
            )
        )
        .apply(disabled_signal_mixin(disabled_signal))
        .apply(invalid_signal_mixin(is_valid_broadcast.signal()))
        .apply_if(class_name.is_some(), |d| d.class(class_name.unwrap()))
    })
}
