use dominator::{html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::always;
use futures_signals::signal::Mutable;

use dominator_material::components::{text_field, CardProps, TextFieldProps};
use dominator_material::utils::mixin::mixin_id;

pub fn input_demo() -> Dom {
    let text_value = Mutable::new("".to_string());

    container!(|d| {
        d.child(card!(CardProps::new()
            .body(static_list!(vec![
                html!("div", {
                    .children(&mut [
                        text_field(TextFieldProps {
                            value: text_value.clone(),
                            assistive_text_signal: Some(Box::new(
                                map_ref!(let cur_val = text_value.signal_cloned() =>
                                    Some(format!("Assistive text - {}", cur_val))))
                            ),
                            ..Default::default()
                        }.label("With dynamic help text"), mixin_id()).0
                    ])
                }),
                html!("div", {  
                    .children(&mut [
                        text_field(TextFieldProps{
                            value: text_value.clone(),
                            error_text_signal: Some(Box::new(always(Some("Only accepts the value `foobar`".to_string())))),
                            ..Default::default()}
                            .label("With error text")
                            .validator(|v| v == "foobar"), mixin_id()).0
                    ])
                }),
                html!("div", {
                    .children(&mut [
                        text_field(TextFieldProps::new(text_value)
                            .label("Always invalid") 
                            .validator(|_| false), mixin_id()).0
                    ])
                }),
            ], mixin_id()),
        ), |v| v.class("demo-card")))
    })
}
