use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals::signal::{
    Broadcaster, BroadcasterSignalCloned, Mutable, MutableSignalCloned, Signal,
};
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

pub struct TextFieldProps<T: Clone, TValidSignal: Signal<Item = bool>> {
    pub label: String,
    pub value: Mutable<T>,
    pub is_valid: TValidSignal,
    pub assistive_text_signal: Option<Box<dyn Signal<Item = Option<String>> + Unpin>>,
    pub error_text_signal: Option<Box<dyn Signal<Item = Option<String>> + Unpin>>,
    pub claim_focus: bool,
}

pub enum InputValue {
    Text(String),
    Bool(bool),
}

impl<
        T: Clone + From<InputValue> + Into<InputValue> + 'static,
        TValidSignal: Signal<Item = bool>,
    > TextFieldProps<T, TValidSignal>
{
    pub fn new(value: Mutable<T>, is_valid: TValidSignal) -> Self {
        TextFieldProps {
            value,
            is_valid,
            label: "".to_string(),
            assistive_text_signal: None,
            error_text_signal: None,
            claim_focus: false,
        }
    }

    #[inline]
    #[must_use]
    pub fn label<TLabel: Into<String>>(mut self, label: TLabel) -> Self {
        self.label = label.into();
        self
    }

    #[inline]
    #[must_use]
    pub fn claim_focus(mut self) -> Self {
        self.claim_focus = true;
        self
    }

    #[inline]
    #[must_use]
    pub fn assistive_text_signal<TSig: Signal<Item = Option<String>> + Unpin + 'static>(
        mut self,
        sig: TSig,
    ) -> Self {
        self.assistive_text_signal = Some(Box::new(sig));
        self
    }

    #[inline]
    #[must_use]
    pub fn error_text_signal<TSig: Signal<Item = Option<String>> + Unpin + 'static>(
        mut self,
        sig: TSig,
    ) -> Self {
        self.error_text_signal = Some(Box::new(sig));
        self
    }
}

pub struct TextFieldOutput<TValidSignal: Signal<Item = bool> + 'static> {
    pub is_valid: BroadcasterSignalCloned<TValidSignal>,
    pub has_focus: MutableSignalCloned<bool>,
}

#[macro_export]
macro_rules! text_field {
    ($props: expr) => {{
        $crate::components::input::text_field::text_field($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::input::text_field::text_field($props, $mixin)
    }};
}

/// Creates a text input element for accepting user data
///
/// The return tuple contains:
/// 0: input Dom entry
/// 1: output of the component, containing a boolean signal for the  validity of the input according to the validator
pub fn text_field<
    T: Clone + From<InputValue> + Into<InputValue> + 'static,
    F,
    TValidSignal: Signal<Item = bool> + 'static,
>(
    props: TextFieldProps<T, TValidSignal>,
    mixin: F,
) -> (Dom, TextFieldOutput<TValidSignal>)
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let has_focus = Mutable::new(false);
    let value = props.value.clone();
    let is_valid_bc = Broadcaster::new(props.is_valid);

    let (input_element, is_valid_bc) =
        text_field_input(&value, &has_focus, props.claim_focus, is_valid_bc);
    let label_element = label_element(&value, &has_focus, props.label.as_str());

    (
        {
            let mut children = vec![input_element, label_element];
            let has_assistive = Mutable::new(false);
            let has_error = Mutable::new(false);

            if let Some(error) = props.error_text_signal {
                let has_error = has_error.clone();

                let error_text_signal = map_ref!(
                    let valid = is_valid_bc.signal_cloned(),
                    let error_text = error => move {
                        if let Some(str) = error_text {
                            if !*valid {
                                has_error.set(true);
                                return Some(crate::text!(str, |d| d.class("dmat-assistive-text").class("dmat-error-text")));
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
                            return Some(crate::text!(str, |d| d.class("dmat-assistive-text")))
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

            html!("div", {
                .child(children)
                .apply(mixin)
                .class_signal(
                    "assistive",
                    map_ref!(
                        let assistive = has_assistive.signal(),
                        let err = has_error.signal() => {
                            *assistive || *err
                        }
                    )
                )
                .class("dmat-input-text-field")
            })
        },
        TextFieldOutput {
            is_valid: is_valid_bc.signal_cloned(),
            has_focus: has_focus.signal_cloned(),
        },
    )
}

#[inline]
fn label_element<T: Clone + From<InputValue> + Into<InputValue> + 'static>(
    value: &Mutable<T>,
    has_focus: &Mutable<bool>,
    label: &str,
) -> Dom {
    html!("span", {
        .class_signal(
            "above",
            clone!(value => map_ref!(
                let focus = has_focus.signal_cloned(),
                let _value = value.signal_cloned() => move {
                    let has_value = match value.get_cloned().into() {
                        InputValue::Text(txt) => !txt.is_empty(),
                        _ => false
                    };

                    *focus || has_value
                })))
        .child(crate::text!(label))
        .class("dmat-input-label-text")
    })
}
#[inline]
fn text_field_input<
    T: Clone + From<InputValue> + Into<InputValue> + 'static,
    TValidSignal: Signal<Item = bool> + 'static,
>(
    value: &Mutable<T>,
    has_focus: &Mutable<bool>,
    claim_focus: bool,
    is_valid_bc: Broadcaster<TValidSignal>,
) -> (Dom, Broadcaster<TValidSignal>) {
    (
        html!("input", {
            .apply_if(claim_focus, clone!(has_focus => move|builder| {
                has_focus.set(true);
                builder.focused(true)
            }))
            .event(clone!(value => move |e: events::Input| {
                #[allow(deprecated)]
                let val =  match e.value() {
                    Some(v) => v.as_str().into(), _ => "".into()
                };

                let val = InputValue::Text(val);
                let val = val.into();

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
            .property_signal("value", value.signal_cloned().map(|v: T| {
                let val: InputValue = v.into();
                val
            }))
            .class_signal("invalid", is_valid_bc.signal_ref(|e| !e))
            .class("dmat-input-element")
        }),
        is_valid_bc,
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
    use futures_signals::signal::{Mutable, SignalExt};
    use futures_util::StreamExt;
    use wasm_bindgen_test::*;

    use crate::components::{text_field, TextFieldProps};

    #[wasm_bindgen_test]
    async fn text_field_validation() {
        let val = Mutable::new("".to_string());

        let field = text_field(
            TextFieldProps {
                label: "".to_string(),
                value: val.clone(),
                is_valid: val.signal_ref(|v| v == "hello"),
                assistive_text_signal: None,
                error_text_signal: None,
                claim_focus: false,
            },
            |d| d.attribute("id", "testfield"),
        );

        let field_dom = field.0;
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
