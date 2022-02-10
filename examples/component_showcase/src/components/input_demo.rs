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
                    label: always(Some("Oranges are the best".to_string())),
                    value: value.clone(),
                    is_valid: value.signal_ref(|v| v == "Orange"),
                    assistive_text_signal: always(None),
                    error_text_signal: always(None),
                    disabled_signal: always(false)
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
                    label: always(Some("Oranges are the best".to_string())),
                    value: value.clone(),
                    is_valid: value.signal_ref(|v| v == "Orange"),
                    assistive_text_signal: always(None),
                    error_text_signal: always(Some("With assistive/error text signal".to_string())),
                    disabled_signal: always(false)
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
                    label: always(Some("Pick one".to_string())),
                    value: value.clone(),
                    is_valid: always(true),
                    assistive_text_signal: always(None),
                    error_text_signal: always(None),
                    disabled_signal: always(false)
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
                    label: always(Some("select with assistive text".to_string())),
                    value: value.clone(),
                    is_valid: value.signal_ref(|v| v == "Banana"),
                    assistive_text_signal: always(Some("This one likes Bananas".to_string())),
                    error_text_signal: always(None),
                    disabled_signal: always(false)
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
                        text_field!(TextFieldProps {
                            claim_focus: true,
                            input_props: InputProps{
                                label: always(Some("With dynamic help text".to_string())),
                                value: value.clone(),
                                is_valid: always(true),
                                assistive_text_signal: map_ref!(let cur_val = value.signal_cloned() =>
                                    Some(format!("Assistive text - {}", cur_val))),
                                error_text_signal: always(None),
                                disabled_signal: always(false)
                            }
                        }).0
                    ])
                }),
                html!("div", {  
                    .children(&mut [
                        text_field!(TextFieldProps {
                                claim_focus: true,
                                input_props: InputProps{
                                    label: always(Some("With error text".to_string())),
                                    value: value.clone(),
                                    is_valid: value.signal_ref(|v| v.contains("foobar")),
                                    assistive_text_signal: map_ref!(let cur_val = value.signal_cloned() =>
                                        Some(format!("Assistive text - {}", cur_val))),
                                    error_text_signal: always(Some("Accepts string containing `foobar`".to_string())),
                                    disabled_signal: always(false)
                                }
                            }).0
                    ])
                }),
                html!("div", {
                    .children(&mut [
                        text_field!(TextFieldProps::new(value.clone(), always(Some("Always invalid".to_string())), always(false), always(None), always(None), always(false))
                            ).0
                    ])
                }),
            ]),
        ), |v| v.class("demo-card"))
}
