use dmat_components::components::layouts::ContentBlockProps;
use dmat_components::components::list::*;
use dmat_components::components::{ButtonStyle, ButtonType, TitleProps};
use dominator::{html, Dom};
use futures_signals::signal::always;
use lipsum::lipsum;

pub fn card_demo() -> Dom {
    let cards = [
        card!({
            .child(content_block!(ContentBlockProps {
                title_section: Some(title!(TitleProps {
                    header_text_signal: always("Card with content block".to_string()),
                    sub_header_text_signal: always(Some("All sections".to_string())),
                })),
                media_section: Some(html!("img", {
                    .attr("src", "images/shapes.svg")
                    .attr("width", "100%")
                    .attr("height", "100%")
                    .attr("alt", "shapes!")
                })),
                supporting_section: Some(text!(lipsum(30))),
                footer_section: Some(button!({
                    .label("Some action")
                    .button_type(ButtonType::Contained)
                    .style(ButtonStyle::Neutral)
                })),
            }))
            .apply(|d| d.style("width", "300px"))
        }),
        card!({
             .child(content_block!(ContentBlockProps {
                title_section: Some(title!(TitleProps {
                    header_text_signal: always("Card without block".to_string()),
                    sub_header_text_signal: always(Some("A sub header".to_string())),
                })),
                media_section: None,
                supporting_section: Some(text!(lipsum(30))),
                footer_section: None,
            }))
            .apply(|d| d.style("width", "300px"))
        }),
        card!({
             .child(content_block!(ContentBlockProps {
                title_section: None,
                media_section: Some(html!("img", {
                    .attr("src", "images/shapes.svg")
                    .attr("width", "100%")
                    .attr("height", "100%")
                    .attr("alt", "shapes!")
                })),
                supporting_section: Some(list!({
                    .rows([
                        title!(TitleProps {
                            header_text_signal: always("Title within supporting".to_string()),
                            sub_header_text_signal: always(None),
                        }),
                        text!(lipsum(30))
                    ])
                })),
                footer_section: None,
            }))
            .apply(|d| d.style("width", "300px"))
        }),
    ];

    container!(|d| d.children(cards))
}
