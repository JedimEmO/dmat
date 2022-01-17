use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use web_sys::HtmlElement;

#[macro_export]
macro_rules! combo_box {
    ($props: expr) => {{
        $crate::components::input::combo_box::combo_box($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::input::combo_box::combo_box($props, $mixin)
    }};
}

pub struct ComboBoxProps {
    pub value: Mutable<String>,
    pub options: MutableVec<String>,
    pub id: String,
    pub valid_signal: Option<Box<dyn Signal<Item = bool> + Unpin>>,
}

#[inline]
pub fn combo_box<F>(props: ComboBoxProps, mixin: F) -> Dom
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("div", {
        .apply(mixin)
        .class("dmat-input-combo-box")
        .children([
            combo_box_input(props.id.as_str(), props.value, props.valid_signal),
            combo_box_datalist(props.id.as_str(), &props.options)
        ])
    })
}

#[inline]
fn combo_box_input(
    data_list_id: &str,
    value: Mutable<String>,
    valid: Option<Box<dyn Signal<Item = bool> + Unpin>>,
) -> Dom {
    html!("input", {
        .class("dmat-input-element")
        .attribute("list", data_list_id)
        .apply_if(valid.is_some(), move |dom_builder| {
            dom_builder.class_signal("-invalid", map_ref! {
                let is_valid = valid.unwrap() => move {
                    !is_valid
                }
            })
        })
        .property_signal("value", value.signal_cloned())
        .event(clone!(value => move |e: events::Input| {
            #[allow(deprecated)]
            if let Some(new_value) = e.value() {
                value.set(new_value);
            }
        }))
    })
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
