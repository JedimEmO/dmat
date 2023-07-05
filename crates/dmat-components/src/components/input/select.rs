use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::signal::{Mutable, Signal};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlSelectElement};

use crate::components::input::input_field::input;
use crate::components::input::input_props::InputProps;

#[macro_export]
macro_rules! select {
    ($props: expr) => {{
        $crate::components::input::select::select($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::input::select::select($props, $mixin)
    }};
}

pub struct SelectProps<
    TLabelSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TValidSignal: Signal<Item = bool> + Unpin + 'static,
    TAssistiveTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TErrorTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TDisabledSignal: Signal<Item = bool> + Unpin + 'static
> {
    pub options: MutableVec<String>,
    pub data_list_id: String,
    pub input_props: InputProps<
        TLabelSignal,
        TValidSignal,
        TAssistiveTextSignal,
        TErrorTextSignal,
        TDisabledSignal
    >,
}

/// The select component is a dropdown from which the user can chose 1 value
#[inline]
pub fn select<
    TLabelSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TValidSignal: Signal<Item = bool> + Unpin + 'static,
    TAssistiveTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TErrorTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TDisabledSignal: Signal<Item = bool> + Unpin + 'static,
    F,
>(
    props: SelectProps<
        TLabelSignal,
        TValidSignal,
        TAssistiveTextSignal,
        TErrorTextSignal,
        TDisabledSignal
    >,
    mixin: F,
) -> Dom
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let has_focus = Mutable::new(true);
    let options = props.options;
    let input_ele = select_input_ele(&props.input_props.value, &options);

    input(
        input_ele,
        &has_focus,
        props.input_props,
        mixin,
        "dmat-input-select",
        None,
    )
}

fn select_input_ele(value: &Mutable<String>, options: &MutableVec<String>) -> Dom {
    html!("select", {
        .prop_signal("value", value.signal_cloned())
        .children_signal_vec(options.signal_vec_cloned().map(|v| html!("option", {
            .text(v.as_str())
            .prop("value", v)
        })))
        .event(clone!(value => move |e: events::Change| {
            #[allow(deprecated)]
            if let Some(target) = e.target() {
                if let Some(target) = target.dyn_ref::<HtmlSelectElement>() {
                    value.set(target.value());
                }
            }
        }))
    })
}
