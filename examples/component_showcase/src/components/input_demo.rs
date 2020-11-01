use dominator::{html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;

use dominator_material::components::layouts::Container;
use dominator_material::components::{Card, List, TextElement};

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
            Card::new()
                .apply(|v| v.class("demo-card"))
                .body(List::new_static(vec![
                    html!("div", {
                        .children(&mut [
                            TextElement::new(self.text_value.clone())
                                .label("Text value")
                                .render(),
                            html!("span", { 
                                .text_signal(self.text_value.signal_cloned().map(|v| format!(" Value: {}", v)))
                            })
                        ])
                    }),
                    html!("div", {
                        .children(&mut [
                            TextElement::new(self.text_value.clone())
                                .label("Invalid")
                                .validator(|_| false)
                                .render()
                        ])
                    }),
                    html!("div", {
                        .children(&mut [
                            TextElement::new(self.text_value.clone())
                                .label("Accepts `foobar`")
                                .validator(|v| v == "foobar")
                                .render()
                        ])
                    })
                ]))
                .render(),
        )
        .render()
    }
}
