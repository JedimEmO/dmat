use dominator::{html, Dom, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::{Broadcaster, Mutable};
use web_sys::HtmlElement;

use crate::components::input::input_props::InputProps;
use crate::components::input::label::label_element;
use crate::components::mixins::children_builder::build_children;
use crate::components::mixins::{
    assistive_text, error_text, with_disabled_signal, with_invalid_signal,
};

pub(crate) fn input<F>(
    input_element: Dom,
    has_focus: &Mutable<bool>,
    props: InputProps,
    mixin: F,
    class_name: &str,
    extra_child: Option<Dom>,
) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let label_element = label_element(input_element, &props.value, &has_focus, props.label);

    let has_assistive = Mutable::new(false);
    let has_error = Mutable::new(false);
    let is_valid = props.is_valid;
    let disabled_signal = props.disabled_signal;

    let is_valid = if let Some(is_valid) = is_valid {
        Some(Broadcaster::new(is_valid))
    } else {
        None
    };

    let error = error_text(
        props.error_text_signal,
        is_valid.as_ref().map(|v| v.signal_cloned()),
        &has_error,
    );

    let assistive = assistive_text(props.assistive_text_signal, &has_assistive);

    let children = build_children(&mut [Some(label_element), extra_child, error, assistive]);

    html!("div", {
        .children(children.into_iter())
        .apply(mixin)
        .class_signal(
            "assistive",
            map_ref!(
                let assistive = has_assistive.signal(),
                let err = has_error.signal() => {
                    *assistive || *err
                }
            )
        )
        .apply(with_disabled_signal(disabled_signal))
        .apply(with_invalid_signal(is_valid.map(|v| v.signal_cloned())))
        .class(class_name)
    })
}
