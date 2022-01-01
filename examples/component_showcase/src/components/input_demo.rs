use dominator::{html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;

use dominator_material::components::layouts::container;
use dominator_material::components::{card, static_list, text_field, CardProps, TextFieldProps};

pub fn input_demo() -> Dom {
    let text_value = Mutable::new("".to_string());

    container(
        card(CardProps::new()
            .with_apply(|v| v.class("demo-card"))
            .body(static_list(vec![
                html!("div", {
                    .children(&mut [
                        text_field(TextFieldProps { value: text_value.clone(), ..Default::default()}.label("Label")).0,
                        html!("span", { 
                            .text_signal(text_value.signal_cloned().map(|v| format!(" Value: {}", v)))
                        })
                    ])
                }),
                html!("div", {
                    .children(&mut [
                        text_field(TextFieldProps::new(text_value.clone())
                            .label("Invalid") 
                            .validator(|_| false)).0
                    ])
                }),
                html!("div", {  
                    .children(&mut [
                        text_field(TextFieldProps::new(text_value.clone())
                            .label("Accepts `foobar`")
                            .validator(|v| v == "foobar")).0
                    ])
                })
            ]),
        )),
    )
}
