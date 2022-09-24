use crate::futures_signals::signal::SignalExt;
use dominator::{html, Dom, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::{Broadcaster, Mutable, Signal};
use web_sys::HtmlElement;

use crate::components::input::input_props::InputProps;
use crate::components::input::label::label_element;
use crate::components::mixins::children_builder::build_children;
use crate::components::mixins::{
    assistive_text, disabled_signal_mixin, error_text, invalid_signal_mixin,
};

pub(crate) fn input<
    TLabelSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TValidSignal: Signal<Item = bool> + Unpin + 'static,
    TAssistiveTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TErrorTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TDisabledSignal: Signal<Item = bool> + Unpin + 'static,
    F,
>(
    input_element: Dom,
    has_focus: &Mutable<bool>,
    props: InputProps<
        TLabelSignal,
        TValidSignal,
        TAssistiveTextSignal,
        TErrorTextSignal,
        TDisabledSignal,
    >,
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

    let is_valid = Broadcaster::new(is_valid);

    let error = error_text(
        props.error_text_signal,
        is_valid.signal_ref(|v| *v),
        &has_error,
    );

    let assistive = assistive_text(props.assistive_text_signal, &has_assistive);

    let children = build_children(&mut [
        Some(label_element),
        extra_child,
        Some(error),
        Some(assistive),
    ]);

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
        .apply(disabled_signal_mixin(disabled_signal))
        .apply(invalid_signal_mixin(is_valid.signal().map(|v| v)))
        .class(class_name)
    })
}