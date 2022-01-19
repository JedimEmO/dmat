use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use web_sys::HtmlElement;

use crate::components::input::label::label_element;

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
    pub label: String,
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
    let (input, has_focus) = combo_box_input(props.id.as_str(), &props.value, props.valid_signal);

    html!("div", {
        .apply(mixin)
        .class("dmat-input-combo-box")
        .child(html!("div", {
            .children([
                label_element(input, &props.value, &has_focus, props.label.as_str()),
                combo_box_datalist(props.id.as_str(), &props.options)
            ])
        }))
    })
}

#[inline]
fn combo_box_input(
    data_list_id: &str,
    value: &Mutable<String>,
    valid: Option<Box<dyn Signal<Item = bool> + Unpin>>,
) -> (Dom, Mutable<bool>) {
    let has_focus = Mutable::new(false);

    (
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
