use dominator::{html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::always;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;

use dominator_material::components::input::input_props::InputProps;
use dominator_material::components::input::ComboBoxProps;
use dominator_material::components::input::SelectProps;
use dominator_material::components::{CardProps, TextFieldProps};

pub fn input_demo() -> Dom {
    let value = Mutable::new("".to_string());
    container!(|d| { d.children(&mut [text_input_demo(&value), combo_box_demo(&value)]) })
}

fn combo_box_demo(value: &Mutable<String>) -> Dom {
    card!(CardProps::new()
        .header(text!("Selection"))
        .body(static_list!(vec![
            combo_box!(ComboBoxProps {
                options: MutableVec::new_with_values(vec![
                    "Banana".to_string(),
                    "Orange".to_string(),
                    "Apple".to_string()
                ]),
                data_list_id: "demo-list-a".into(),
                input_props: InputProps {
                    label: Some(Box::new(always("Oranges are the best".to_string()))),
                    value: value.clone(),
                    is_valid: Some(Box::new(value.signal_ref(|v| v == "Orange"))),
                    assistive_text_signal: None,
                    error_text_signal: None
                }
            }),
            combo_box!(ComboBoxProps {
                options: MutableVec::new_with_values(vec![
                    "Banana".to_string(),
                    "Orange".to_string(),
                    "Apple".to_string()
                ]),
                data_list_id: "demo-list-a".into(),
                input_props: InputProps {
                    label: Some(Box::new(always("Oranges are the best".to_string()))),
                    value: value.clone(),
                    is_valid: Some(Box::new(value.signal_ref(|v| v == "Orange"))),
                    assistive_text_signal: None,
                    error_text_signal: Some(Box::new(always(Some(
                        "With assistive/error text signal".to_string()
                    ))))
                }
            }),
            select!(SelectProps {
                data_list_id: "data-list-b".to_string(),
                options: MutableVec::new_with_values(vec![
                    "Banana".to_string(),
                    "Orange".to_string(),
                    "Apple".to_string()
                ]),
                input_props: InputProps {
                    label: Some(Box::new(always("Pick one".to_string()))),
                    value: value.clone(),
                    is_valid: None,
                    assistive_text_signal: None,
                    error_text_signal: None
                }
            }),
            select!(SelectProps {
                data_list_id: "data-list-c".to_string(),
                options: MutableVec::new_with_values(vec![
                    "Banana".to_string(),
                    "Orange".to_string(),
                    "Apple".to_string()
                ]),
                input_props: InputProps {
                    label: Some(Box::new(always("select with assistive text".to_string()))),
                    value: value.clone(),
                    is_valid: Some(Box::new(value.signal_ref(|v| v == "Banana"))),
                    assistive_text_signal: Some(Box::new(always(Some(
                        "This one likes Bananas".to_string()
                    )))),
                    error_text_signal: None
                }
            })
        ])))
}
fn text_input_demo(value: &Mutable<String>) -> Dom {
    card!(CardProps::new()
        .header(text!("Text field"))
            .body(static_list!(vec![
                html!("div", {
                    .children(&mut [
                        text_field!(TextFieldProps::new(value.clone())
                            .assistive_text_signal(map_ref!(let cur_val = value.signal_cloned() =>
                                    Some(format!("Assistive text - {}", cur_val))))
                            .claim_focus()
                            .validator(value.signal_ref(|_| true))
                            .label("With dynamic help text")).0
                    ])
                }),
                html!("div", {  
                    .children(&mut [
                        text_field!(
                            TextFieldProps::new(value.clone())
                            .label("With error text")
                            .validator(value.signal_ref(|v| v.contains("foobar")))
                            .error_text_signal(always(Some("Accepts string containing `foobar`".to_string())))
                            ).0
                    ])
                }),
                html!("div", {
                    .children(&mut [
                        text_field!(TextFieldProps::new(value.clone())
                            .validator(value.signal_ref(|_| false))
                            .label("Always invalid")).0
                    ])
                }),
            ]),
        ), |v| v.class("demo-card"))
}
