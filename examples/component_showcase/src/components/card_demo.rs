use dominator::{Dom, html};

use dominator_material::components::{Button, Card, ButtonType};

pub struct CardDemo {}

impl CardDemo {
    pub fn new() -> CardDemo {
        CardDemo {}
    }

    pub fn render(self) -> Dom {
        html!("div", {
            .class("demo-cards")
            .children(&mut [
                Card::new(html!("div", { .text("This is the body") }))
                    .header(html!("div", { .text("A header text") }))
                    .footer(html!("div", {
                        .class("demo-buttons")
                        .children(&mut [
                            html!("div", { .class("demo-button") .child(Button::new().text("A button").button_type(ButtonType::Text).render()) }),
                            html!("div", { .class("demo-button") .child(Button::new().text("Another button").button_type(ButtonType::Text).render()) }),
                        ])
                    }))
                    .render(),
                Card::new(html!("div", { .text("Only a body") }))
                    .render()
            ])
        })
    }
}
