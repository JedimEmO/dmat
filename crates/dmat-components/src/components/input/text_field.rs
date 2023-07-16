use crate::components::input::input_field::{input_wrapper, InputWrapperProps};
use dominator::{clone, events, html, Dom};
use futures_signals::signal::{Mutable, MutableSignalCloned, Signal, SignalExt};

#[component(render_fn = text_field)]
pub struct TextField<TOnValuePickCb: Fn(String) = fn(String) -> ()> {
    #[default(| _ | {})]
    on_value_change: TOnValuePickCb,

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

    #[default(false)]
    claim_focus: bool,
}

pub struct TextFieldOutput {
    pub has_focus: MutableSignalCloned<bool>,
}

/// Creates a text input element for accepting user data
///
/// The return tuple contains:
/// 0: input Dom entry
/// 1: output of the component, containing a boolean signal for the  validity of the input according to the validator
pub fn text_field(props: impl TextFieldPropsTrait + 'static) -> (Dom, TextFieldOutput) {
    let TextFieldProps {
        on_value_change,
        label,
        value,
        is_valid,
        disabled,
        assistive_text,
        error_text,
        claim_focus,
        apply,
    } = props.take();

    let value_bc = value.broadcast();
    let has_focus = Mutable::new(false);
    let input_element = text_field_input(
        value_bc.signal_ref(|v| v.clone()),
        on_value_change,
        &has_focus,
        claim_focus,
    );

    (
        input_wrapper(
            InputWrapperProps::new()
                .value_signal(value_bc.signal_ref(|v| v.clone()))
                .input(input_element)
                .has_focus_signal(has_focus.signal())
                .apply(|d| if let Some(a) = apply { a(d) } else { d })
                .has_focus_signal(has_focus.signal())
                .class_name("dmat-input-text-field".to_string())
                .error_text_signal(error_text)
                .assistive_text_signal(assistive_text)
                .disabled_signal(disabled)
                .is_valid_signal(is_valid)
                .label_signal(label),
        ),
        TextFieldOutput {
            has_focus: has_focus.signal_cloned(),
        },
    )
}

#[inline]
fn text_field_input(
    value_signal: impl Signal<Item = String> + 'static,
    on_value_change: impl Fn(String) + 'static,
    has_focus: &Mutable<bool>,
    claim_focus: bool,
) -> Dom {
    html!("input", {
        .apply_if(claim_focus, clone!(has_focus => move|builder| {
            has_focus.set(true);
            builder.focused(true)
        }))
        .event(move |e: events::Input| {
            #[allow(deprecated)]
            if let Some(val) = e.value() {
                on_value_change(val)
            };

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
        .prop_signal("value", value_signal)
        .class("dmat-input-element")
    })
}

#[cfg(test)]
mod test {
    use futures_signals::signal::Mutable;
    use wasm_bindgen_test::*;

    use crate::components::{text_field, TextFieldProps};

    #[wasm_bindgen_test]
    async fn text_field_validation() {
        let val = Mutable::new("".to_string());

        let field = text_field!({
            .value_signal(val.signal_cloned())
            .is_valid_signal(val.signal_ref(|v| v == "hello"))
            .apply(|d| d.attr("id", "testfield"))
        });

        let field_dom = field.0;
        let _field_out = field.1;

        dominator::append_dom(
            &web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap(),
            field_dom,
        );

        val.set("hello".to_string());

        // let mut valid_stream = field_out.is_valid.to_stream();
        //
        // while !valid_stream.next().await.unwrap() {}
    }
}
