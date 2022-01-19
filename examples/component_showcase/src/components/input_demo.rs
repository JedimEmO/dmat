use dominator::{html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::always;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;

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
                label: "Oranges are the best".to_string(),
                value: value.clone(),
                options: MutableVec::new_with_values(vec![
                    "Banana".to_string(),
                    "Orange".to_string(),
                    "Apple".to_string()
                ]),
                id: "demo-list-a".into(),
                valid_signal: Some(Box::new(value.signal_ref(|v| v == "Orange")))
            }),
            select!(SelectProps {
                label: "Pick one".to_string(),
                value: value.clone(),
                options: MutableVec::new_with_values(vec![
                    "Banana".to_string(),
                    "Orange".to_string(),
                    "Apple".to_string()
                ]),
                id: "demo-list-b".into(),
                valid_signal: None
            })
        ])))
}
fn text_input_demo(value: &Mutable<String>) -> Dom {
    card!(CardProps::new()
        .header(text!("Text field"))
            .body(static_list!(vec![
                html!("div", {
                    .children(&mut [
                        text_field!(TextFieldProps::new(value.clone(), value.signal_ref(|_| true))
                            .assistive_text_signal(map_ref!(let cur_val = value.signal_cloned() =>
                                    Some(format!("Assistive text - {}", cur_val))))
                            .claim_focus()
                            .label("With dynamic help text")).0
                    ])
                }),
                html!("div", {  
                    .children(&mut [
                        text_field!(
                            TextFieldProps::new(value.clone(), value.signal_ref(|v| v.contains("foobar")))
                            .label("With error text")
                            .error_text_signal(always(Some("Accepts string containing `foobar`".to_string())))
                            ).0
                    ])
                }),
                html!("div", {
                    .children(&mut [
                        text_field!(TextFieldProps::new(value.clone(), value.signal_ref(|_| false))
                            .label("Always invalid")).0
                    ])
                }),
            ]),
        ), |v| v.class("demo-card"))
}
