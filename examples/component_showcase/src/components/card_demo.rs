

use dominator::{html, Dom};



use dominator_material::components::{button, card, text, ButtonProps, ButtonType, CardProps};
use dominator_material::utils::component_signal::once_cmp;

pub struct CardDemo {}

impl CardDemo {
    pub fn new() -> CardDemo {
        CardDemo {}
    }

    pub fn render(self) -> Dom {
        let cards  = vec![
            card(CardProps {
                header: Some(html!("span", {.text("functional card 2")})),
                ..Default::default()
            }),
            card(CardProps::new()
                .with_header(html!("div", { .text("A header element") }))
                .with_body(html!("div", { .text("This is the body") }))
                .with_footer(html!("div", {
                            .class("demo-buttons")
                            .children(&mut [
                                html!("div", { .class("demo-button") .child(button(ButtonProps::new(once_cmp(text("A button"))).button_type(ButtonType::Text))) }),
                                html!("div", { .class("demo-button") .child(button(ButtonProps::new(once_cmp(text("Another button"))).button_type(ButtonType::Text))) }),
                            ])
                        }))),
            card(CardProps::new()
                .with_body(html!("div", { .text("Only a body") })) 
                ),

            card(CardProps::new()
                .with_title("With a title", Some("and a sub title"))
                .with_body(html!("div", { .text("This card has a title. It is mutually exclusive with the header element") }))
                )
        ];

        card(
            CardProps::new()
                .with_apply(|v| v.class("demo-cards").class("demo-card"))
                .with_body(html!("div", {
                    .children(cards.into_iter())
                })),
        )
    }
}
