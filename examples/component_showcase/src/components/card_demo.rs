use dominator::{html, Dom};

use dominator_material::components::{button, card, text, ButtonProps, ButtonType, CardProps};
use dominator_material::utils::mixin::{mixin_id, no_mixin};

pub fn card_demo() -> Dom {
    let cards = vec![
        card(CardProps {
            header_view: Some(html!("span", {.text("functional card 2")}).into()),
            ..Default::default()
        }, mixin_id()),
        card(CardProps::new()
            .header(html!("div", { .text("A header element") }))
            .body(html!("div", { .text("This is the body") }), )
            .footer(html!("div", {
                            .class("demo-buttons")
                            .children(&mut [
                                text("Footer", no_mixin),
                                html!("div", { .class("demo-button") .child(button(ButtonProps::new().content(text("A button", no_mixin)).button_type(ButtonType::Text), mixin_id())) }),
                                html!("div", { .class("demo-button") .child(button(ButtonProps::new().content(text("Another button", no_mixin)).button_type(ButtonType::Text), mixin_id())) }),
                            ])
                        })), mixin_id()),
        card(CardProps::new()
            .body(html!("div", { .text("Only a body") }), )
             , mixin_id()),

        card(CardProps::new()
            .with_title("With a title", Some("and a sub title"))
            .body(html!("div", { .text("This card has a title. It is mutually exclusive with the header element") }),),
        mixin_id())
    ];

    card(
        CardProps::new().body(html!("div", {
            .children(cards.into_iter())
        })),
        |v| v.class("demo-cards").class("demo-card"),
    )
}
