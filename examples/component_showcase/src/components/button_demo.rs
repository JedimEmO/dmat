use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::always;
use futures_signals::signal::Mutable;

use dominator_material::components::{ButtonProps, ButtonStyle, ButtonType, CardProps};

pub fn button_demo() -> Dom {
    let counter = Mutable::new(0);

    container!(|d| {
        d.children(&mut[
                card!(CardProps::new()
                    .header(html!("div", { .text("ButtonType::Contained") }))
                    .body(
                        static_list!(vec![
                            button!(ButtonProps::new(|_|{}, always(false))
                                .style(ButtonStyle::Prominent)
                                .content(text!("prominent"))),
                            button!(ButtonProps::new(|_|{}, always(false))
                                .style(ButtonStyle::Neutral)
                                .content(text!("neutral"))),
                            button!(ButtonProps::new(|_|{}, always(false))
                                .style(ButtonStyle::Unimportant)
                                .content(text!("unimportant")))
                    ])
                )),

            card!(CardProps::new()
                    .header(html!("div", { .text("ButtonType::Elevated") }))
                    .body(
                        static_list!(vec![
                            button!(ButtonProps::new(|_|{}, always(false))
                                .style(ButtonStyle::Prominent)
                                .button_type(ButtonType::Elevated)
                                .content(text!("prominent"))),
                            button!(ButtonProps::new(|_|{}, always(false))
                                .style(ButtonStyle::Neutral)
                                .button_type(ButtonType::Elevated)
                                .content(text!("neutral"))),
                            button!(ButtonProps::new(|_|{}, always(false))
                                .style(ButtonStyle::Unimportant)
                                .button_type(ButtonType::Elevated)
                                .content(text!("unimportant")))
                    ])
                )),
                card!(CardProps::new()
                    .header(html!("div", { .text("ButtonType::Text") }))
                    .body(static_list!(vec![
                            button!(ButtonProps::new(|_|{}, always(false))
                                .style(ButtonStyle::Prominent)
                                .button_type(ButtonType::Text)
                                .content(text!("prominent"))),
                            button!(ButtonProps::new(|_|{}, always(false))
                                .style(ButtonStyle::Neutral)
                                .button_type(ButtonType::Text)
                                .content(text!("neutral"))),
                            button!(ButtonProps::new(|_|{}, always(false))
                                .style(ButtonStyle::Unimportant)
                                .button_type(ButtonType::Text)
                                .content(text!("unimportant")))
                    ]
                ))),
                card!(CardProps::new()
                    .header(html!("div", { .text("ButtonType::Outlined") }))
                    .body(static_list!(vec![
                            button!(ButtonProps::new(|_|{}, always(false))
                                .style(ButtonStyle::Prominent)
                                .button_type(ButtonType::Outlined)
                                .content(text!("prominent"))),
                            button!(ButtonProps::new(|_|{}, always(false))
                                .style(ButtonStyle::Neutral)
                                .button_type(ButtonType::Outlined)
                                .content(text!("neutral"))),
                            button!(ButtonProps::new(|_|{}, always(false))
                                .style(ButtonStyle::Unimportant)
                                .button_type(ButtonType::Outlined)
                                .content(text!("unimportant")))
                    ]
                ))),
                card!(CardProps::new()
                    .header(dynamic_text!(map_ref! {
                        let value = counter.signal() => format!("Button with dynamic content -  value is {}", value)
                    }))
                    .body(
                        button!(
                            ButtonProps::new(clone!(counter => move |_| {
                                let v = *counter.lock_ref();
                                *counter.lock_mut() = v + 1;
                            }), always(false))
                            .content(dynamic_text!(map_ref! {
                                let value = counter.signal() => format!("Clicked {} times", value)
                            })))))
            ])
    })
}
