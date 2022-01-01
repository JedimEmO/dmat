use dominator::{html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;

use dominator_material::components::layouts::container;
use dominator_material::components::{card, static_list, text_field, CardProps, TextFieldProps};
use dominator_material::utils::component_signal::ComponentSignal;

pub fn input_demo() -> Dom {
    let text_value = Mutable::new("".to_string());

    container(
        card(CardProps::new()
            .with_apply(|v| v.class("demo-card"))
            .body(static_list(vec![
                html!("div", {
                    .children(&mut [
                        text_field(TextFieldProps { value: text_value.clone(), ..Default::default()}.label("Label")).0.into_dom(),
                        html!("span", { 
                            .text_signal(text_value.signal_cloned().map(|v| format!(" Value: {}", v)))
                        })
                    ])
                }),
                html!("div", {  
                    .children(&mut [
                        text_field(TextFieldProps{
                            value: text_value.clone(),
                            error_message_signal_factory: Some(Box::new(move |is_valid| {
                                ComponentSignal::from_signal(is_valid.map(|valid| {
                                    match valid {
                                        true => None,
                                        false => Some(html!("span", {
                                            .text("Only accepts the value `foobar`")
                                        }))
                                    }
                                }))
                            })) ,
                            ..Default::default()}
                            .label("Accepts `foobar`")
                            .validator(|v| v == "foobar")).0.into_dom()
                    ])
                }),
                html!("div", {
                    .children(&mut [
                        text_field(TextFieldProps::new(text_value.clone())
                            .label("Always invalid") 
                            .validator(|_| false)).0.into_dom()
                    ])
                }),
            ]),
        )),
    )
}
