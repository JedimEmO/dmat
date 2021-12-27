use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::Mutable;
use futures_signals::signal::{MutableSignal, SignalExt};
use futures_util::future::ready;
use wasm_bindgen::JsValue;
use wasm_bindgen::__rt::std::rc::Rc;
use web_sys::HtmlElement;

#[derive(Default)]
pub struct TextElementProps<T: Clone> {
    pub label: Option<String>,
    pub value: Mutable<T>,
    pub id: Option<String>,
    pub validator: Option<Rc<dyn Fn(&T) -> bool>>,
    pub depends_on: Mutable<()>,
    pub has_focus: Mutable<bool>,
    pub apply: Option<Box<dyn FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
}

pub enum InputValue {
    Text(String),
    Bool(bool),
}

impl<T: Clone + From<InputValue> + Into<InputValue> + 'static> TextElementProps<T> {
    pub fn new(value: Mutable<T>) -> Self {
        TextElementProps {
            value,
            label: None,
            id: None,
            validator: None,
            depends_on: Mutable::new(()),
            has_focus: Mutable::new(false),
            apply: None,
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

    pub fn id(mut self: Self, id: &str) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_apply<F: 'static>(mut self, f: F) -> Self
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    {
        self.apply = Some(Box::new(f));
        self
    }
}

pub struct TextElementOutput {
    pub is_valid: MutableSignal<bool>,
}

/// Creates a text input element for accepting user data
///
/// The return tuple contains:
/// 0: input Dom entry
/// 1: output of the component, containing a boolean signal for the  validity of the input according to the validator
pub fn text_element<T: Clone + From<InputValue> + Into<InputValue> + 'static>(
    props: TextElementProps<T>,
) -> (Dom, TextElementOutput) {
    let is_valid = Mutable::new(true);

    (
        clone!(is_valid => Dom::with_state(props, move |field_props| {
            let id = match &field_props.id {
                Some(v) => v.clone(),
                _ => "".into(),
            };

            let validator = field_props.validator.clone();
            let depends_on = field_props.depends_on.clone();
            let has_focus = field_props.has_focus.clone();
            let value = field_props.value.clone();

            let validate = clone!(validator, is_valid => move |val: &T| {
                if let Some(validator_inner) = &validator {
                    is_valid.replace(validator_inner(val));
                } else {
                    is_valid.replace(true);
                }
            });

            validate(&field_props.value.get_cloned());

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
                .attribute("id", id.as_str())
                .property_signal("value", field_props.value.signal_cloned().map(|v: T| {
                    let val: InputValue = v.into();
                    val
                }))
                .class_signal("invalid", is_valid.signal_cloned().map(|e| !e))
                .class("dmat-input-element")
            });

            let mut children = match &field_props.label {
                Some(label) => vec![
                    input,
                    html!("label", {
                        .text(label.as_str())
                        .attribute("for", id.as_str())
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
                        .class("dmat-floating-label")
                    }),
                ],
                _ => vec![input],
            };

            html!("div", {
                .apply_if(field_props.apply.is_some(), |dom_builder| {
                    (field_props.apply.take().unwrap())(dom_builder)
                })
                .children(children.as_mut_slice())
                .class("dmat-input")
            })
        })),
        TextElementOutput {
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
