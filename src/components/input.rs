use dominator::{clone, events, html, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::{always, Mutable, MutableSignalCloned};
use futures_signals::signal::{MutableSignal, SignalExt};
use futures_util::future::ready;
use wasm_bindgen::JsValue;
use wasm_bindgen::__rt::std::rc::Rc;
use web_sys::Element;

use crate::components::text;
use crate::elements::elements::new_html;
use crate::utils::component_signal::ComponentSignal;

#[derive(Default)]
pub struct TextFieldProps<T: Clone> {
    pub label: Option<String>,
    pub value: Mutable<T>,
    pub validator: Option<Rc<dyn Fn(&T) -> bool>>,
    pub depends_on: Mutable<()>,
    pub has_focus: Mutable<bool>,
    pub error_message_signal_factory:
        Option<Box<dyn FnOnce(MutableSignalCloned<bool>) -> ComponentSignal>>,
}

pub enum InputValue {
    Text(String),
    Bool(bool),
}

impl<T: Clone + From<InputValue> + Into<InputValue> + 'static> TextFieldProps<T> {
    pub fn new(value: Mutable<T>) -> Self {
        TextFieldProps {
            value,
            label: None,
            validator: None,
            depends_on: Mutable::new(()),
            has_focus: Mutable::new(false),
            error_message_signal_factory: None,
        }
    }

    pub fn depends_on(mut self: Self, depends_on: Mutable<()>) -> Self {
        self.depends_on = depends_on;
        self
    }

    pub fn validator<F>(mut self: Self, validator: F) -> Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        self.validator = Some(Rc::new(validator));
        self
    }

    pub fn label(mut self: Self, label: &str) -> Self {
        self.label = Some(label.into());
        self
    }
}

pub struct TextFieldOutput {
    pub is_valid: MutableSignal<bool>,
}

/// Creates a text input element for accepting user data
///
/// The return tuple contains:
/// 0: input Dom entry
/// 1: output of the component, containing a boolean signal for the  validity of the input according to the validator
pub fn text_field<T: Clone + From<InputValue> + Into<InputValue> + 'static>(
    props: TextFieldProps<T>,
) -> (DomBuilder<Element>, TextFieldOutput) {
    let is_valid = Mutable::new(true);

    let error_text_signal = match props.error_message_signal_factory {
        Some(factory) => (factory)(is_valid.signal_cloned()),
        _ => ComponentSignal(Box::new(always(None))),
    };

    (
        {
            let validator = props.validator.clone();
            let depends_on = props.depends_on.clone();
            let has_focus = props.has_focus.clone();
            let value = props.value.clone();

            let validate = clone!(validator, is_valid => move |val: &T| {
                if let Some(validator_inner) = &validator {
                    is_valid.replace(validator_inner(val));
                } else {
                    is_valid.replace(true);
                }
            });

            validate(&props.value.get_cloned());

            let input = html!("input", {
                .future(clone!(validate, value, depends_on => async move {
                    let deps = map_ref!(
                        let _deps = depends_on.signal(),
                        let val =  value.signal_cloned() => move {
                            validate(&val);
                        }
                    );

                    // Trigger validate every time a dependency changes
                    deps.for_each(|_| {
                        ready(())
                    }).await;
                }))
                .event(clone!(validate, value => move |e: events::Input| {
                    let val =  match e.value() {
                        Some(v) => v.as_str().into(), _ => "".into()
                    };

                    let val = InputValue::Text(val);
                    let val = val.into();

                    validate(&val);
                    value.replace(val);
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
                .property_signal("value", props.value.signal_cloned().map(|v: T| {
                    let val: InputValue = v.into();
                    val
                }))
                .class_signal("invalid", is_valid.signal_cloned().map(|e| !e))
                .class("dmat-input-element")
            });

            let error_text = html!("span", {
                .child_signal(error_text_signal.0)
                .class("dmat-error-text")
            });

            let mut children = match &props.label {
                Some(label) => vec![html!("label", {
                    .children(&mut [
                        input,
                        text(label.as_str())
                        .class_signal("above",
                                clone!(has_focus, value => map_ref!(
                                    let focus = has_focus.signal_cloned(),
                                    let _value = value.signal_cloned() => move {
                                        let has_value = match value.get_cloned().into() {
                                            InputValue::Text(txt) => txt.len() > 0,
                                            _ => false
                                        };

                                        *focus || has_value
                                    }
                                ))
                            )
                            .class("dmat-input-label-text").into_dom(),
                        error_text
                    ])
                    .class("dmat-floating-label")
                })],
                _ => vec![input, error_text],
            };

            new_html("div")
                .children(children.as_mut_slice())
                .class("dmat-input")
        },
        TextFieldOutput {
            is_valid: is_valid.signal(),
        },
    )
}

impl From<InputValue> for String {
    fn from(val: InputValue) -> Self {
        match val {
            InputValue::Text(v) => v,
            InputValue::Bool(v) => match v {
                true => "true".to_string(),
                _ => "false".to_string(),
            },
        }
    }
}

impl From<String> for InputValue {
    fn from(val: String) -> Self {
        InputValue::Text(val)
    }
}

impl From<InputValue> for JsValue {
    fn from(value: InputValue) -> Self {
        match value {
            InputValue::Text(v) => v.into(),
            InputValue::Bool(v) => v.into(),
        }
    }
}
