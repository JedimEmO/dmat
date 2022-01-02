use dominator::{html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::always;
use futures_signals::signal::Mutable;

use dominator_material::components::layouts::container;
use dominator_material::components::{card, static_list, text_field, CardProps, TextFieldProps};

pub fn input_demo() -> Dom {
    let text_value = Mutable::new("".to_string());

    container(
        card(CardProps::new()
            .body(static_list(vec![
                html!("div", {
                    .children(&mut [
                        text_field(TextFieldProps {
                            value: text_value.clone(),
                            assistive_text_signal: Some(Box::new(
                                map_ref!(let cur_val = text_value.signal_cloned() =>
                                    Some(format!("Assistive text - {}", cur_val))))
                            ),
                            ..Default::default()
                        }.label("With dynamic help text")).0.into_dom()
                    ])
                }),
                html!("div", {  
                    .children(&mut [
                        text_field(TextFieldProps{
                            value: text_value.clone(),
                            error_text_signal: Some(Box::new(always(Some("Only accepts the value `foobar`".to_string())))),
                            ..Default::default()}
                            .label("With error text")
                            .validator(|v| v == "foobar")).0.into_dom()
                    ])
                }),
                html!("div", {
                    .children(&mut [
                        text_field(TextFieldProps::new(text_value)
                            .label("Always invalid") 
                            .validator(|_| false)).0.into_dom()
                    ])
                }),
            ]),
        )).apply(|v| v.class("demo-card")).into_dom()
    )
}
