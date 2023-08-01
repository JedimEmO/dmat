use dmat_components::components::layouts::*;
use dmat_components::components::*;
use dominator::{html, Dom};
use lipsum::lipsum;

pub fn card_demo() -> Dom {
    let cards = [
        card!({
            .child(content_block!(ContentBlockProps {
                title_section: Some(title!({
                    .header_text("Card with content block".to_string())
                    .sub_header_text(Some("All sections".to_string()))
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
                title_section: Some(title!({
                    .header_text("Card without block".to_string())
                    .sub_header_text(Some("A sub header".to_string()))
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
                        title!( {
                            .header_text("Title within supporting".to_string())
                        }),
                        text!(lipsum(30))
                    ])
                })),
                footer_section: None,
            }))
            .apply(|d| d.style("width", "300px"))
        }),
    ];

    container!({.children(cards)})
}
