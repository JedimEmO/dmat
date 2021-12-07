use dominator::{clone, events, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::Mutable;
use futures_signals::signal::{MutableSignal, SignalExt};
use futures_util::future::ready;
use wasm_bindgen::JsValue;
use wasm_bindgen::__rt::std::rc::Rc;

#[derive(Clone, Default)]
pub struct TextElementProps<T: Clone> {
    pub label: Option<String>,
    pub value: Mutable<T>,
    pub id: Option<String>,
    pub validator: Option<Rc<dyn Fn(&T) -> bool>>,
    pub depends_on: Mutable<()>,
    pub has_focus: Mutable<bool>,
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
}

pub struct TextElementOutput {
    pub is_valid: MutableSignal<bool>,
}

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

            let validate = clone!(field_props, is_valid => move |val: &T| {
                if let Some(validator) = &field_props.validator {
                    is_valid.replace(validator(val));
                } else {
                    is_valid.replace(true);
                }
            });

            validate(&field_props.value.get_cloned());

            let input = html!("input", {
                .future(clone!(field_props, validate => async move {
                    let deps = map_ref!(
                        let _deps = field_props.depends_on.signal(),
                        let _val =  field_props.value.signal_cloned() => move {
                            validate(&field_props.value.get_cloned());
                        }
                    );

                    // Trigger validate every time a dependency changes
                    deps.for_each(|_| {
                        ready(())
                    }).await;
                }))
                .event(clone!(validate, field_props => move |e: events::Input| {
                    let val =  match e.value() {
                        Some(v) => v.as_str().into(), _ => "".into()
                    };

                    let val = InputValue::Text(val);
                    let val = val.into();

                    validate(&val);
                    field_props.value.replace(val);
                }))
                .event(clone!(field_props => {
                    move |_e: events::Focus| {
                        field_props.has_focus.set(true);
                    }
                }))
                .event(clone!(field_props => {
                    move |_: events::Blur| {
                        field_props.has_focus.set(false);
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
                            clone!(field_props => map_ref!(
                                let focus = field_props.has_focus.signal_cloned(),
                                let _value = field_props.value.signal_cloned() => move {
                                    let has_value = match field_props.value.get_cloned().into() {
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
