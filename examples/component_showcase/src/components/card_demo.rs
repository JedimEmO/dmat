use dominator::{html, Dom};

use dominator_material::components::{button, card, text, ButtonProps, ButtonType, CardProps};
pub fn card_demo() -> Dom {
    let cards  = vec![
        card(CardProps {
            header_view: Some(html!("span", {.text("functional card 2")}).into()),
            ..Default::default()
        }),
        card(CardProps::new()
            .header(html!("div", { .text("A header element") }))
            .body(html!("div", { .text("This is the body") }), )
            .footer(html!("div", {
                            .class("demo-buttons")
                            .children(&mut [
                                text("Footer").into_dom(),
                                html!("div", { .class("demo-button") .child(button(ButtonProps::new().content(text("A button")).button_type(ButtonType::Text))) }),
                                html!("div", { .class("demo-button") .child(button(ButtonProps::new().content(text("Another button")).button_type(ButtonType::Text))) }),
                            ])
                        }))),
        card(CardProps::new()
            .body(html!("div", { .text("Only a body") }), )
        ),

        card(CardProps::new()
            .with_title("With a title", Some("and a sub title"))
            .body(html!("div", { .text("This card has a title. It is mutually exclusive with the header element") }), )
        )
    ];

    card(
        CardProps::new()
            .with_apply(|v| v.class("demo-cards").class("demo-card"))
            .body(html!("div", {
                .children(cards.into_iter())
            })),
    )
}
