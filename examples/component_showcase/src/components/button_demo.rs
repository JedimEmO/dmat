use dominator::{clone, Dom};
use futures_signals::map_ref;
use futures_signals::signal::always;
use futures_signals::signal::Mutable;

use dmat_components::components::layouts::ContentBlockProps;
use dmat_components::components::TitleProps;
use dmat_components::components::{ButtonStyle, ButtonType};

pub fn button_demo() -> Dom {
    let counter = Mutable::new(0);

    container!(|d| {
        d.children(&mut[
                card!(
                content_block!(
                    ContentBlockProps {
                        title_section: Some(title!(TitleProps {
                            header_text_signal: always("ButtonType::Contained".to_string()),
                            sub_header_text_signal: always(None)
                        })),
                        media_section: Some(static_list!(vec![
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
                            ])),
                        supporting_section: None,
                        footer_section: None
                    })
                ),

            card!(content_block!(
                    ContentBlockProps {
                        title_section: Some(title!(TitleProps {
                            header_text_signal: always("ButtonType::Elevated".to_string()),
                            sub_header_text_signal: always(None)
                        })),
                        media_section: Some(static_list!(vec![
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
                        ])),
                        supporting_section: None,
                        footer_section: None
                    })),
                card!(content_block!(
                    ContentBlockProps {
                        title_section: Some(title!(TitleProps {
                            header_text_signal: always("ButtonType::Text".to_string()),
                            sub_header_text_signal: always(None)
                        })),
                        media_section: Some(static_list!(vec![
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
                            ]
                        )),
                        supporting_section: None,
                        footer_section: None
                    })),
                card!(content_block!(
                    ContentBlockProps {
                        title_section: Some(title!(TitleProps {
                            header_text_signal: always("ButtonType::Outlined".to_string()),
                            sub_header_text_signal: always(None)
                        })),
                        media_section: Some(static_list!(vec![
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
                            ]
                        )),
                        supporting_section: None,
                        footer_section: None
                    })),
                card!(content_block!(
                    ContentBlockProps {
                        title_section: Some(title!(TitleProps {
                            header_text_signal: always("Button with dynamic content".to_string()),
                            sub_header_text_signal: map_ref! {
                                let value = counter.signal() => Some(format!("Button with dynamic content -  value is {}", value))
                            }
                        })),
                        media_section: Some(static_list!([
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
                        ),
                        supporting_section: None,
                        footer_section: None
                    }))
            ])
    })
}
