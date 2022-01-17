use dominator::{html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::always;
use futures_signals::signal::Mutable;

use dominator_material::components::{CardProps, TextFieldProps};

pub fn input_demo() -> Dom {
    let text_value = Mutable::new("".to_string());

    container!(|d| {
        d.child(card!(CardProps::new()
            .body(static_list!(vec![
                html!("div", {
                    .children(&mut [
                        text_field!(TextFieldProps::new(text_value.clone(), text_value.signal_ref(|_| true))
                            .assistive_text_signal(map_ref!(let cur_val = text_value.signal_cloned() =>
                                    Some(format!("Assistive text - {}", cur_val))))
                            .claim_focus()
                            .label("With dynamic help text")).0
                    ])
                }),
                html!("div", {  
                    .children(&mut [
                        text_field!(
                            TextFieldProps::new(text_value.clone(), text_value.signal_ref(|v| v == "foobar"))
                            .label("With error text")
                            .error_text_signal(always(Some("Only accepts the value `foobar`".to_string())))
                            ).0
                    ])
                }),
                html!("div", {
                    .children(&mut [
                        text_field!(TextFieldProps::new(text_value.clone(), text_value.signal_ref(|_| false))
                            .label("Always invalid")).0
                    ])
                }),
            ]),
        ), |v| v.class("demo-card")))
    })
}
