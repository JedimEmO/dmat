use dmat_components::components::layouts::ContentBlockProps;
use dmat_components::components::{
    ButtonContent, ButtonProps, ButtonStyle, ButtonType, TitleProps,
};
use dominator::{html, Dom};
use futures_signals::signal::always;
use lipsum::lipsum;

pub fn card_demo() -> Dom {
    let cards = [
        card!(
            content_block!(ContentBlockProps {
                title_section: Some(title!(TitleProps {
                    header_text_signal: always("Card with content block".to_string()),
                    sub_header_text_signal: always(Some("All sections".to_string())),
                })),
                media_section: Some(html!("img", {
                    .attribute("src", "images/shapes.svg")
                    .attribute("width", "100%")
                    .attribute("height", "100%")
                    .attribute("alt", "shapes!")
                })),
                supporting_section: Some(text!(lipsum(30))),
                footer_section: Some(button!(ButtonProps {
                    content: Some(ButtonContent::Label("Some action".to_string())),
                    button_type: ButtonType::Contained,
                    style: ButtonStyle::Neutral,
                    disabled_signal: always(false),
                    click_handler: |_| {}
                })),
            }),
            |d| d.style("width", "300px")
        ),
        card!(
            content_block!(ContentBlockProps {
                title_section: Some(title!(TitleProps {
                    header_text_signal: always("Card without block".to_string()),
                    sub_header_text_signal: always(Some("A sub header".to_string())),
                })),
                media_section: None,
                supporting_section: Some(text!(lipsum(30))),
                footer_section: None,
            }),
            |d| d.style("width", "300px")
        ),
        card!(
            content_block!(ContentBlockProps {
                title_section: None,
                media_section: Some(html!("img", {
                    .attribute("src", "images/shapes.svg")
                    .attribute("width", "100%")
                    .attribute("height", "100%")
                    .attribute("alt", "shapes!")
                })),
                supporting_section: Some(static_list!([
                    title!(TitleProps {
                        header_text_signal: always("Title within supporting".to_string()),
                        sub_header_text_signal: always(None),
                    }),
                    text!(lipsum(30))
                ])),
                footer_section: None,
            }),
            |d| d.style("width", "300px")
        ),
    ];

    container!(|d| d.children(cards))
}
