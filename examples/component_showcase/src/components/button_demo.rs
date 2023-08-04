use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::Mutable;

use dmat_components::components::layouts::*;
use dmat_components::components::*;

pub fn button_demo() -> Dom {
    let counter = Mutable::new(0);

    container!({
        .children([
                card!({
                .child(content_block!({
                        .title_section(Some(title!({
                            .header_text("ButtonType::Contained".to_string())
                            .sub_header_text(None)
                        })))
                        .media_section(Some(list!({
                            .items([
                                button!({
                                    .style(ButtonStyle::Prominent)
                                    .content(html!("span", { .text("prominent") }))
                                }),
                                button!({
                                    .style(ButtonStyle::Neutral)
                                    .content(html!("span", { .text("neutral") }))
                                }),
                                button!({
                                    .style(ButtonStyle::Unimportant)
                                    .content(html!("span", { .text("unimportant") }))
                                })
                            ])
                        })))
                    }))
            }),

            card!({.child(content_block!({
                        .title_section(Some(title!( {
                            .header_text("ButtonType::Elevated".to_string())
                        })))
                        .media_section(Some(list!({
                                .items([
                                    button!({
                                        .style(ButtonStyle::Prominent)
                                        .button_type(ButtonType::Elevated)
                                        .content(html!("span", { .text("prominent") }))
                                    }),
                                    button!({
                                        .style(ButtonStyle::Neutral)
                                        .button_type(ButtonType::Elevated)
                                        .content(html!("span", { .text("neutral") }))
                                    }),
                                    button!({
                                        .style(ButtonStyle::Unimportant)
                                        .button_type(ButtonType::Elevated)
                                        .content(html!("span", { .text("unimportant") }))
                                })
                            ])
                        })))
                    }))}),
                card!({.child(content_block!({
                        .title_section(Some(title!( {
                            .header_text("ButtonType::Text".to_string())
                        })))
                        .media_section(Some(list!({
                            .items([
                                button!({
                                    .style(ButtonStyle::Prominent)
                                    .button_type(ButtonType::Text)
                                    .content(html!("span", { .text("prominent") }))
                               }),
                                button!({
                                    .style(ButtonStyle::Neutral)
                                    .button_type(ButtonType::Text)
                                    .content(html!("span", { .text("neutral") }))
                                }),
                                button!({
                                    .style(ButtonStyle::Unimportant)
                                    .button_type(ButtonType::Text)
                                    .content(html!("span", { .text("unimportant") }))
                                })
                            ])
                        })))
                    }))}),
                card!({.child(content_block!({
                        .title_section(Some(title!({
                            .header_text("ButtonType::Outlined".to_string())
                        })))
                        .media_section(Some(list!({
                            .items([
                                button!({
                                    .style(ButtonStyle::Prominent)
                                    .button_type(ButtonType::Outlined)
                                    .content(html!("span", { .text("prominent") }))
                                }),
                                button!({
                                    .style(ButtonStyle::Neutral)
                                    .button_type(ButtonType::Outlined)
                                    .content(html!("span", { .text("neutral") }))
                                }),
                                button!({
                                    .style(ButtonStyle::Unimportant)
                                    .button_type(ButtonType::Outlined)
                                    .content(html!("span", { .text("unimportant") }))
                                })
                            ])
                        })))
                    }))}),
                card!({.child(content_block!({
                        .title_section(Some(title!({
                            .header_text("Button with dynamic content".to_string())
                            .sub_header_text_signal(map_ref! {
                                let value = counter.signal() => Some(format!("Button with dynamic content -  value is {}", value))
                            })
                        })))
                        .media_section(Some(list!({
                             .items([
                                button!({
                                    .click_handler(clone!(counter => move |_| {
                                        let v = *counter.lock_ref();
                                        *counter.lock_mut() = v + 1;
                                    }))
                                    .content(html!("span", {.text_signal(map_ref! {
                                        let value = counter.signal() => format!("Clicked {} times", value)
                                    })}))
                                })
                            ])
                        })))
                    }))})
            ])
    })
}
