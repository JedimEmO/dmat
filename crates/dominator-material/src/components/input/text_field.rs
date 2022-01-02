use dominator::{clone, events, html, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal};
use futures_signals::signal::{MutableSignal, SignalExt};
use futures_util::future::ready;
use wasm_bindgen::JsValue;
use wasm_bindgen::__rt::std::rc::Rc;
use web_sys::Element;

use crate::components::text;
use crate::elements::new_html::new_html;

#[derive(Default)]
pub struct TextFieldProps<T: Clone> {
    pub label: String,
    pub value: Mutable<T>,
    pub validator: Option<Rc<dyn Fn(&T) -> bool>>,
    pub depends_on: Mutable<()>,
    pub has_focus: Mutable<bool>,
    pub assistive_text_signal: Option<Box<dyn Signal<Item = Option<String>> + Unpin>>,
    pub error_text_signal: Option<Box<dyn Signal<Item = Option<String>> + Unpin>>,
}

pub enum InputValue {
    Text(String),
    Bool(bool),
}

impl<T: Clone + From<InputValue> + Into<InputValue> + 'static> TextFieldProps<T> {
    pub fn new(value: Mutable<T>) -> Self {
        TextFieldProps {
            value,
            label: "".to_string(),
            validator: None,
            depends_on: Mutable::new(()),
            has_focus: Mutable::new(false),
            assistive_text_signal: None,
            error_text_signal: None,
        }
    }

    #[inline]
    #[must_use]
    pub fn depends_on(mut self, depends_on: Mutable<()>) -> Self {
        self.depends_on = depends_on;
        self
    }

    #[inline]
    #[must_use]
    pub fn validator<F>(mut self, validator: F) -> Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        self.validator = Some(Rc::new(validator));
        self
    }

    #[inline]
    #[must_use]
    pub fn label<TLabel: Into<String>>(mut self, label: TLabel) -> Self {
        self.label = label.into();
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
                            validate(val);
                        }
                    );

                    // Trigger validate every time a dependency changes
                    deps.for_each(|_| {
                        ready(())
                    }).await;
                }))
                .event(clone!(validate, value => move |e: events::Input| {
                    #[allow(deprecated)]
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

            let label_element = html!("span", {
                                .class_signal(
                            "above",
                            clone!(has_focus, value => map_ref!(
                                let focus = has_focus.signal_cloned(),
                                let _value = value.signal_cloned() => move {
                                    let has_value = match value.get_cloned().into() {
                                        InputValue::Text(txt) => !txt.is_empty(),
                                        _ => false
                                    };

                                    *focus || has_value
                                })))
                .child(text(props.label.as_str()).into_dom())
                .class("dmat-input-label-text")
            });

            let mut children = vec![input, label_element];
            let has_assistive = Mutable::new(false);
            let has_error = Mutable::new(false);

            if let Some(error) = props.error_text_signal {
                let has_error = has_error.clone();

                let error_text_signal = map_ref!(
                    let valid = is_valid.signal_cloned(),
                    let error_text = error => move {
                        if let Some(str) = error_text {
                            if !*valid {
                                has_error.set(true);
                                return Some(text(str).class("dmat-assistive-text").class("dmat-error-text").into_dom());
                            }
                        }

                        has_error.set(false);

                        None
                    }
                );

                children.push(html!("span", {
                    .child_signal(error_text_signal)
                }));
            }

            if let Some(assistive) = props.assistive_text_signal {
                let has_assistive = has_assistive.clone();
                let assistive_element_signal = map_ref!(
                    let assistive_text = assistive => move {
                        let ass = has_assistive.clone();

                        if let Some(str) = assistive_text {
                            ass.set(true);
                            return Some(text(str).class("dmat-assistive-text").into_dom())
                        }

                        ass.set(false);
                        None
                    }
                );

                children.push(html!("span", {
                    .child_signal(assistive_element_signal)
                }));
            }

            let children = html!("label", {
                .children(children.as_mut_slice())
                .class("dmat-floating-label")
            });

            new_html("div")
                .child(children)
                .class("dmat-input-text-field")
                .class_signal(
                    "assistive",
                    map_ref!(
                        let assistive = has_assistive.signal(),
                        let err = has_error.signal() => {
                            *assistive || *err
                        }
                    ),
                )
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

#[cfg(test)]
mod test {
    use crate::components::{text_field, TextFieldProps};
    use futures_signals::signal::{Mutable, SignalExt};
    use futures_util::StreamExt;
    use std::default::Default;
    use std::rc::Rc;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn text_field_validation() {
        let val = Mutable::new("".to_string());

        let field = text_field(TextFieldProps {
            value: val.clone(),
            validator: Some(Rc::new(|v| v == "hello")),
            ..Default::default()
        });

        let field_dom = field.0.attribute("id", "testfield").into_dom();
        let field_out = field.1;

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

        let mut valid_stream = field_out.is_valid.to_stream();

        while !valid_stream.next().await.unwrap() {}
    }
}
