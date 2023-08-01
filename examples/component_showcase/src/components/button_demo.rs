use dominator::{clone, Dom};
use futures_signals::map_ref;
use futures_signals::signal::Mutable;

use dmat_components::components::layouts::*;
use dmat_components::components::*;

pub fn button_demo() -> Dom {
    let counter = Mutable::new(0);

    container!({
        .children([
                card!({
                .child(content_block!(
                    ContentBlockProps {
                        title_section: Some(title!({
                            .header_text("ButtonType::Contained".to_string())
                            .sub_header_text(None)
                        })),
                        media_section: Some(list!({
                            .rows([
                                button!({
                                    .style(ButtonStyle::Prominent)
                                    .content(text!("prominent"))
                                }),
                                button!({
                                    .style(ButtonStyle::Neutral)
                                    .content(text!("neutral"))
                                }),
                                button!({
                                    .style(ButtonStyle::Unimportant)
                                    .content(text!("unimportant"))
                                })
                            ])
                        })),
                        supporting_section: None,
                        footer_section: None
                    }))
            }),

            card!({.child(content_block!(
                    ContentBlockProps {
                        title_section: Some(title!( {
                            .header_text("ButtonType::Elevated".to_string())
                        })),
                        media_section: Some(list!({
                                .rows([
                                    button!({
                                        .style(ButtonStyle::Prominent)
                                        .button_type(ButtonType::Elevated)
                                        .content(text!("prominent"))
                                    }),
                                    button!({
                                        .style(ButtonStyle::Neutral)
                                        .button_type(ButtonType::Elevated)
                                        .content(text!("neutral"))
                                    }),
                                    button!({
                                        .style(ButtonStyle::Unimportant)
                                        .button_type(ButtonType::Elevated)
                                        .content(text!("unimportant"))
                                })
                            ])
                        })),
                        supporting_section: None,
                        footer_section: None
                    }))}),
                card!({.child(content_block!(
                    ContentBlockProps {
                        title_section: Some(title!( {
                            .header_text("ButtonType::Text".to_string())
                        })),
                        media_section: Some(list!({
                            .rows([
                                button!({
                                    .style(ButtonStyle::Prominent)
                                    .button_type(ButtonType::Text)
                                    .content(text!("prominent"))
                               }),
                                button!({
                                    .style(ButtonStyle::Neutral)
                                    .button_type(ButtonType::Text)
                                    .content(text!("neutral"))
                                }),
                                button!({
                                    .style(ButtonStyle::Unimportant)
                                    .button_type(ButtonType::Text)
                                    .content(text!("unimportant"))
                                })
                            ])
                        })),
                        supporting_section: None,
                        footer_section: None
                    }))}),
                card!({.child(content_block!(
                    ContentBlockProps {
                        title_section: Some(title!({
                            .header_text("ButtonType::Outlined".to_string())
                        })),
                        media_section: Some(list!({
                            .rows([
                                button!({
                                    .style(ButtonStyle::Prominent)
                                    .button_type(ButtonType::Outlined)
                                    .content(text!("prominent"))
                                }),
                                button!({
                                    .style(ButtonStyle::Neutral)
                                    .button_type(ButtonType::Outlined)
                                    .content(text!("neutral"))
                                }),
                                button!({
                                    .style(ButtonStyle::Unimportant)
                                    .button_type(ButtonType::Outlined)
                                    .content(text!("unimportant"))
                                })
                            ])
                        })),
                        supporting_section: None,
                        footer_section: None
                    }))}),
                card!({.child(content_block!(
                    ContentBlockProps {
                        title_section: Some(title!({
                            .header_text("Button with dynamic content".to_string())
                            .sub_header_text_signal(map_ref! {
                                let value = counter.signal() => Some(format!("Button with dynamic content -  value is {}", value))
                            })
                        })),
                        media_section: Some(list!({
                             .rows([
                                button!({
                                    .click_handler(clone!(counter => move |_| {
                                        let v = *counter.lock_ref();
                                        *counter.lock_mut() = v + 1;
                                    }))
                                    .content(dynamic_text!(map_ref! {
                                        let value = counter.signal() => format!("Clicked {} times", value)
                                    }))
                                })
                            ])
                        })),
                        supporting_section: None,
                        footer_section: None
                    }))})
            ])
    })
}
