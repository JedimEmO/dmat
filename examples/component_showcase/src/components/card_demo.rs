use dominator::{html, Dom};
use dominator_material::components::{ButtonProps, ButtonType, CardProps};
use dominator_material::utils::mixin::id_attribute_mixin;
use futures_signals::signal::always;

pub fn card_demo() -> Dom {
    let mut cards = [
        card!(CardProps {
            header_view: Some(html!("span", {.text("functional card 2")}).into()),
            ..Default::default()
        }),
        card!(CardProps::new()
            .header(html!("div", { .text("A header element") }))
            .body(html!("div", { .text("This is the body") }))
            .footer(html!("div", {
                .class("demo-buttons")
                .children(&mut [
                    text!("Footer"),
                    button!(
                        ButtonProps::new(|_|{}, always(false))
                            .content(text!("A button"))
                            .button_type(ButtonType::Text),
                        id_attribute_mixin("demo-button")),
                    button!(
                        ButtonProps::new(|_|{}, always(false))
                            .content(text!("Another button"))
                            .button_type(ButtonType::Text),
                        id_attribute_mixin("demo-button"))

                ])
            }))),
        card!(CardProps::new().body(html!("div", {
            .text("Only a body")
        }),)),
        card!(CardProps::new()
            .with_title("With a title", Some("and a sub title"))
            .body(html!("div", {
                .text("This card has a title. It is mutually exclusive with the header element")
            }))),
    ];

    container!(|d| d.children(&mut cards))
}
