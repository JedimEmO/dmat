use crate::components::input::label::label_element;
use crate::components::input::ComboBoxProps;
use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlSelectElement};

pub type SelectProps = ComboBoxProps;

#[macro_export]
macro_rules! select {
    ($props: expr) => {{
        $crate::components::input::select::select($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::input::select::select($props, $mixin)
    }};
}

pub fn select<F>(props: SelectProps, mixin: F) -> Dom
where
    F: Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let value = props.value;
    let has_focus = Mutable::new(true);
    let options = props.options;
    let input = select_input_ele(&value, &options);
    let label = label_element(input, &value, &has_focus, props.label.as_str());

    html!("div", {
        .apply(mixin)
        .class("dmat-input-select")
        .child(label)
        .class_signal("-invalid", map_ref! {
            let cur_val =  value.signal_cloned() => move {
                !options.lock_ref().contains(cur_val)
            }
        })
    })
}

fn select_input_ele(value: &Mutable<String>, options: &MutableVec<String>) -> Dom {
    html!("select", {
        .property_signal("value", value.signal_cloned())
        .children_signal_vec(options.signal_vec_cloned().map(|v| html!("option", {
            .text(v.as_str())
            .property("value", v)
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
