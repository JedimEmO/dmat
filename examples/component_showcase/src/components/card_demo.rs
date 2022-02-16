use dominator::Dom;
use dominator_material::components::layouts::ContentBlockProps;
use dominator_material::components::TitleProps;
use futures_signals::signal::always;

pub fn card_demo() -> Dom {
    let mut cards = [
        card!(content_block!(ContentBlockProps {
            title_section: Some(title!(TitleProps {
                header_text_signal: always("Card with content block".to_string()),
                sub_header_text_signal: always(Some("A sub header".to_string())),
            })),
            media_section: None,
            footer_section: None,
            supporting_section: None,
        })),
        // card!(CardProps::new()
        //     .header(html!("div", { .text("A header element") }))
        //     .body(html!("div", { .text("This is the body") }))
        //     .footer(html!("div", {
        //         .class("demo-buttons")
        //         .children(&mut [
        //             text!("Footer"),
        //             button!(
        //                 ButtonProps::new(|_|{}, always(false))
        //                     .content(text!("A button"))
        //                     .button_type(ButtonType::Text),
        //                 id_attribute_mixin("demo-button")),
        //             button!(
        //                 ButtonProps::new(|_|{}, always(false))
        //                     .content(text!("Another button"))
        //                     .button_type(ButtonType::Text),
        //                 id_attribute_mixin("demo-button"))
        //
        //         ])
        //     }))),
        // card!(CardProps::new().body(html!("div", {
        //     .text("Only a body")
        // }),)),
        // card!(CardProps::new()
        //     .with_title("With a title", Some("and a sub title"))
        //     .body(html!("div", {
        //         .text("This card has a title. It is mutually exclusive with the header element")
        //     }))),
    ];

    container!(|d| d.children(&mut cards))
}
