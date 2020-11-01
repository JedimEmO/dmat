use dominator::{html, Dom};

use dominator_material::components::{Button, ButtonType, Card};

pub struct CardDemo {}

impl CardDemo {
    pub fn new() -> CardDemo {
        CardDemo {}
    }

    pub fn render(self) -> Dom {
        let cards  = vec![
            Card::new()
                .header(html!("div", { .text("A header element") }))
                .body(html!("div", { .text("This is the body") }))
                .footer(html!("div", {
                            .class("demo-buttons")
                            .children(&mut [
                                html!("div", { .class("demo-button") .child(Button::new().text("A button").button_type(ButtonType::Text).render()) }),
                                html!("div", { .class("demo-button") .child(Button::new().text("Another button").button_type(ButtonType::Text).render()) }),
                            ])
                        })).render(),
            Card::new()
                .body(html!("div", { .text("Only a body") })) 
                .render(),

            Card::new()
                .title("With a title", Some("and a sub title"))
                .body(html!("div", { .text("This card has a title. It is mutually exclusive with the header element") }))
                .render()
        ];

        Card::new()
            .apply(|v| v.class("demo-cards").class("demo-card"))
            .body(html!("div", {
                .children(cards.into_iter())
            }))
            .render()
    }
}
