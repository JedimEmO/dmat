use dominator::{html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::always;

use dominator_material::components::layouts::Container;
use dominator_material::components::{card, list, text_element, CardProps, TextElementProps};

pub struct InputDemo {
    text_value: Mutable<String>,
}

impl InputDemo {
    pub fn new() -> InputDemo {
        InputDemo {
            text_value: Mutable::new("".into()),
        }
    }

    pub fn render(self) -> Dom {
        Container::new(
            card(CardProps::new()
                .with_apply(|v| v.class("demo-card"))
                .with_body(list(always(vec![
                    html!("div", {
                        .children(&mut [
                            text_element(TextElementProps { value: self.text_value.clone(), ..Default::default()}).0,
                            html!("span", { 
                                .text_signal(self.text_value.signal_cloned().map(|v| format!(" Value: {}", v)))
                            })
                        ])
                    }),
                    html!("div", {
                        .children(&mut [
                            text_element(TextElementProps::new(self.text_value.clone())
                                .label("Invalid") 
                                .validator(|_| false)).0
                        ])
                    }),
                    html!("div", {  
                        .children(&mut [
                            text_element(TextElementProps::new(self.text_value.clone())
                                .label("Accepts `foobar`")
                                .validator(|v| v == "foobar")).0
                        ])
                    })
                ])))),
        )
        .render()
    }
}
