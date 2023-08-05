use crate::components::input::input_field::{input_wrapper, InputWrapperProps};
use crate::components::input::validation_result::ValidationResult;
use dominator::{events, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

pub struct SelectOption {
    pub value: String,
    pub display_text: String,
}

#[component(render_fn = select)]
pub struct Select<TOnValuePickCb: Fn(String) = fn(String) -> ()> {
    #[signal_vec]
    #[default(vec ! [])]
    options: SelectOption,

    #[default(|_| {})]
    on_change: TOnValuePickCb,

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

    #[default(None)]
    input_id: Option<String>,
}

/// The select component is a dropdown from which the user can chose 1 value
#[inline]
pub fn select(props: impl SelectPropsTrait + 'static) -> Dom {
    let SelectProps {
        options,
        on_change,
        label,
        value,
        is_valid,
        disabled,
        assistive_text,
        input_id,
        apply,
    } = props.take();

    let value_bc = value.broadcast();
    let input_ele = select_input_ele(
        value_bc.signal_ref(|v| v.clone()),
        on_change,
        options,
        input_id.clone(),
    );

    input_wrapper(
        InputWrapperProps::new()
            .input(input_ele)
            .has_focus(true)
            .class_name("dmat-input-select".to_string())
            .apply(|d| if let Some(a) = apply { d.apply(a) } else { d })
            .assistive_text_signal(assistive_text)
            .disabled_signal(disabled)
            .is_valid_signal(is_valid)
            .label_signal(label)
            .value_signal(value_bc.signal_ref(|v| v.clone()))
            .input_id(input_id),
    )
}

fn select_input_ele(
    value_signal: impl Signal<Item = String> + 'static,
    value_change_cb: impl Fn(String) + 'static,
    options: impl SignalVec<Item = SelectOption> + 'static,
    input_id: Option<String>,
) -> Dom {
    html!("select", {
        .apply_if(input_id.is_some(), |dom| {
            dom.attr("id", input_id.unwrap().as_str())
        })
        .prop_signal("value", value_signal)
        .children_signal_vec(options.map(|v| html!("option", {
            .text(v.display_text.as_str())
            .prop("value", v.value)
        })))
        .event(move |e: events::Change| {
            #[allow(deprecated)]
            if let Some(target) = e.target() {
                if let Some(target) = target.dyn_ref::<HtmlSelectElement>() {
                    value_change_cb(target.value());
                }
            }
        })
    })
}
