use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::signal::{Mutable, Signal};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use web_sys::HtmlElement;

use crate::components::input::input::input;
use crate::components::input::input_props::InputProps;

#[macro_export]
macro_rules! combo_box {
    ($props: expr) => {{
        $crate::components::input::combo_box::combo_box($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::input::combo_box::combo_box($props, $mixin)
    }};
}

pub struct ComboBoxProps<
    TLabelSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TValidSignal: Signal<Item = bool> + Unpin + 'static,
    TAssistiveTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TErrorTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TDisabledSignal: Signal<Item = bool> + Unpin + 'static,
> {
    pub options: MutableVec<String>,
    pub data_list_id: String,
    pub input_props: InputProps<
        TLabelSignal,
        TValidSignal,
        TAssistiveTextSignal,
        TErrorTextSignal,
        TDisabledSignal,
    >,
}

#[inline]
pub fn combo_box<
    TLabelSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TValidSignal: Signal<Item = bool> + Unpin + 'static,
    TAssistiveTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TErrorTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TDisabledSignal: Signal<Item = bool> + Unpin + 'static,
    F,
>(
    props: ComboBoxProps<
        TLabelSignal,
        TValidSignal,
        TAssistiveTextSignal,
        TErrorTextSignal,
        TDisabledSignal,
    >,
    mixin: F,
) -> Dom
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let (combo_input, has_focus) =
        combo_box_input(props.data_list_id.as_str(), &props.input_props.value);

    input(
        combo_input,
        &has_focus,
        props.input_props,
        mixin,
        "dmat-input-combo-box",
        Some(combo_box_datalist(
            props.data_list_id.as_str(),
            &props.options,
        )),
    )
}

#[inline]
fn combo_box_input(data_list_id: &str, value: &Mutable<String>) -> (Dom, Mutable<bool>) {
    let has_focus = Mutable::new(false);

    (
        html!("input", {
            .class("dmat-input-element")
            .attribute("list", data_list_id)
            .property_signal("value", value.signal_cloned())
            .event(clone!(value => move |e: events::Input| {
                #[allow(deprecated)]
                if let Some(new_value) = e.value() {
                    value.set(new_value);
                }
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
        }),
        has_focus,
    )
}

#[inline]
fn combo_box_datalist(data_list_id: &str, options: &MutableVec<String>) -> Dom {
    html!("datalist", {
        .attribute("id", data_list_id)
        .children_signal_vec(options.signal_vec_cloned().map(|v| html!("option", {
            .property("value", v)
        })))
    })
}
