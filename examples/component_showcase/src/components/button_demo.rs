use dominator::{clone, html, Dom};

use dominator_material::components::{ButtonProps, ButtonStyle, ButtonType, CardProps};

use futures_signals::map_ref;
use futures_signals::signal::Mutable;
pub fn button_demo() -> Dom {
    let counter = Mutable::new(0);

    container!(|d| {
        d.children(&mut[
                card!(CardProps::new()
                    .header(html!("div", { .text("ButtonType::Contained") }))
                    .body(
                        static_list!(vec![
                            button!(ButtonProps::new()
                                .style(ButtonStyle::Prominent)
                                .content(text!("prominent"))),
                            button!(ButtonProps::new()
                                .style(ButtonStyle::Neutral)
                                .content(text!("neutral"))),
                            button!(ButtonProps::new()
                                .style(ButtonStyle::Unimportant)
                                .content(text!("unimportant")))
                    ])
                )),
                card!(CardProps::new()
                    .header(html!("div", { .text("ButtonType::Text") }))
                    .body(static_list!(vec![
                            button!(ButtonProps::new()
                                .style(ButtonStyle::Prominent)
                                .button_type(ButtonType::Text)
                                .content(text!("prominent"))),
                            button!(ButtonProps::new()
                                .style(ButtonStyle::Neutral)
                                .button_type(ButtonType::Text)
                                .content(text!("neutral"))),
                            button!(ButtonProps::new()
                                .style(ButtonStyle::Unimportant)
                                .button_type(ButtonType::Text)
                                .content(text!("unimportant")))
                    ]
                ))),
                card!(CardProps::new()
                    .header(html!("div", { .text("ButtonType::Outlined") }))
                    .body(static_list!(vec![
                            button!(ButtonProps::new()
                                .style(ButtonStyle::Prominent)
                                .button_type(ButtonType::Outlined)
                                .content(text!("prominent"))),
                            button!(ButtonProps::new()
                                .style(ButtonStyle::Neutral)
                                .button_type(ButtonType::Outlined)
                                .content(text!("neutral"))),
                            button!(ButtonProps::new()
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
                            ButtonProps::new()
                            .content(dynamic_text!(map_ref! {
                                let value = counter.signal() => format!("Clicked {} times", value)
                            }))
                            .on_click(clone!(counter => move |_| {
                                let v = *counter.lock_ref();
                                *counter.lock_mut() = v + 1;
                            })))))
            ])
    })
}
