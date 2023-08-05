use dominator::{clone, events, html, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{SignalVec, SignalVecExt};

use crate::components::input::input_field::{input_wrapper, InputWrapperProps};
use crate::components::input::validation_result::ValidationResult;
use crate::components::input::SelectOption;

#[component(render_fn = combo_box)]
struct ComboBox<TOnValuePickCb: Fn(String) = fn(String) -> ()> {
    data_list_id: String,
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

/// The combo box is a searchable dropdown or text field with hints, depending on your point of view.
/// It renders as a text field in which the user can type freely, but it will have a filtered list
/// of options in a dropdown to select from as well.
#[inline]
pub fn combo_box(props: impl ComboBoxPropsTrait + 'static) -> Dom {
    let ComboBoxProps {
        data_list_id,
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

    let (combo_input, has_focus) = combo_box_input(
        data_list_id.clone().unwrap().as_str(),
        value_bc.signal_ref(|v| v.clone()),
        on_change,
        input_id.clone(),
    );

    input_wrapper(
        InputWrapperProps::new()
            .input(combo_input)
            .has_focus_signal(has_focus.signal())
            .class_name("dmat-input-combo-box".to_string())
            .extra_child(combo_box_datalist(data_list_id.unwrap().as_str(), options))
            .apply(|d| if let Some(a) = apply { d.apply(a) } else { d })
            .assistive_text_signal(assistive_text)
            .disabled_signal(disabled)
            .is_valid_signal(is_valid)
            .label_signal(label)
            .value_signal(value_bc.signal_ref(|v| v.clone()))
            .input_id(input_id),
    )
}

#[inline]
fn combo_box_input(
    data_list_id: &str,
    value_signal: impl Signal<Item = String> + 'static,
    on_change: impl Fn(String) + 'static,
    input_id: Option<String>,
) -> (Dom, Mutable<bool>) {
    let has_focus = Mutable::new(false);

    (
        html!("input", {
            .apply_if(input_id.is_some(), |dom_builder| {
                dom_builder.attr("id", input_id.unwrap().as_str())
            })
            .class("dmat-input-element")
            .attr("list", data_list_id)
            .prop_signal("value", value_signal)
            .event(move |e: events::Input| {
                #[allow(deprecated)]
                if let Some(new_value) = e.value() {
                    on_change(new_value);
                }
            })
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
fn combo_box_datalist(
    data_list_id: &str,
    options: impl SignalVec<Item = SelectOption> + 'static,
) -> Dom {
    html!("datalist", {
        .attr("id", data_list_id)
        .children_signal_vec(options.map(|v| html!("option", {
            .apply_if(v.value != v.display_text, |d | d.text(v.display_text.as_str()))
            .prop("value", v.value)
        })))
    })
}
