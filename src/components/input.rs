use dominator::{clone, Dom, events, html};

use futures_signals::signal::{Mutable};
use futures_signals::signal::SignalExt;
use futures_util::future::{ready};
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen::JsValue;

#[derive(Clone)]
pub struct TextElement<T: Clone> {
    label: Option<String>,
    value: Mutable<T>,
    id: Option<String>,
    is_valid: Mutable<bool>,
    validator: Option<Rc<dyn Fn(&T) -> bool>>,
    depends_on: Mutable<()>,
    has_focus: Mutable<bool>
}

pub enum InputValue {
    Text(String),
    Bool(bool),
}

impl<T: Clone + From<InputValue> + Into<InputValue> + 'static> TextElement<T> {
    pub fn new(value: Mutable<T>) -> Rc<Self> {
        Rc::new(TextElement {
            value,
            label: None,
            id: None,
            is_valid: Mutable::new(true),
            validator: None,
            depends_on: Mutable::new(()),
            has_focus: Mutable::new(false)
        })
    }

    pub fn depends_on(mut self: Rc<Self>, depends_on: Mutable<()>) -> Rc<Self> {
        Rc::get_mut(&mut self).unwrap().depends_on = depends_on;
        self
    }

    pub fn validator(mut self: Rc<Self>, validator: Rc<dyn Fn(&T) -> bool>) -> Rc<Self> {
        Rc::get_mut(&mut self).unwrap().validator = Some(validator);
        self
    }

    pub fn label(mut self: Rc<Self>, label: &str) -> Rc<Self> {
        Rc::get_mut(&mut self).unwrap().label = Some(label.into());
        self
    }

    pub fn id(mut self: Rc<Self>, id: &str) -> Rc<Self> {
        Rc::get_mut(&mut self).unwrap().id = Some(id.into());
        self
    }

    pub(crate) fn validate(&self, val: &T) {
        if let Some(validator) = &self.validator {
            self.is_valid.replace(validator(&val));
        }
    }

    pub fn render(self: Rc<Self>) -> Dom {
        text_element::<T>(self)
    }
}

#[inline]
fn text_element<T: Clone + From<InputValue> + Into<InputValue> + 'static>(field: Rc<TextElement<T>>) -> Dom {
    Dom::with_state(field, |field| {
        let id = match &field.id {
            Some(v) => v.clone(),
            _ => "".into()
        };

        field.validate(&field.value.get_cloned());

        let input = html!("input", {
            .future(clone!(field => async move {
                field.depends_on.signal().for_each(|_| {
                    field.validate(&field.value.get_cloned());
                    ready(())
                }).await;
            }))
            .event(clone!(field => move |e: events::Input| {
                let val =  match e.value() {
                    Some(v) => v.as_str().into(), _ => "".into()
                };

                let val = InputValue::Text(val);
                let val = val.into();

                field.validate(&val);
                field.value.replace(val);
            }))
            .event(clone!(field => {
                move |_e: events::Focus| {
                    field.has_focus.set(true);
                }
            }))
            .event(clone!(field => {
                move |_: events::Blur| {
                    field.has_focus.set(false);
                }
            }))
            .attribute("id", id.as_str())
            .property_signal("value", field.value.signal_cloned().map(|v: T| {
                let val: InputValue = v.into();
                val
            }))
            .class_signal("invalid", field.is_valid.signal_cloned().map(|e| !e))
            .class("dmat-input-element")
        });

        let mut children = match &field.label {
            Some(label) => vec![
                input,
                html!("label", {
                    .text(label.as_str())
                    .attribute("for", id.as_str())
                    .class_signal("above", field.has_focus.signal().map(clone!(field => move|focus| {
                        let has_value = match field.value.get_cloned().into() {
                            InputValue::Text(txt) => txt.len() > 0,
                            _ => false
                        };

                        focus || has_value
                    })))
                    .class("dmat-floating-label")
                })
            ],
            _ => vec![input]
        };

        html!("div", {
            .children(children.as_mut_slice())
            .class("dmat-input")
        })
    })
}

impl From<InputValue> for String {
    fn from(val: InputValue) -> Self {
        match val {
            InputValue::Text(v) => v,
            InputValue::Bool(v) => match v {
                true => "true".to_string(),
                _ => "false".to_string()
            }
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
            InputValue::Bool(v) => v.into()
        }
    }
}
