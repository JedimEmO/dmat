use dominator::{clone, Dom};
use futures_signals::map_ref;
use futures_signals::signal::always;
use futures_signals::signal::Mutable;

use dominator_material::components::{ButtonProps, ButtonStyle, ButtonType};

pub fn button_demo() -> Dom {
    let counter = Mutable::new(0);

    container!(|d| {
        d.children(&mut[
                card!(
                static_list!(vec![
                    text!("ButtonType::Contained"),
                    button!(ButtonProps::new(|_|{}, always(false))
                        .style(ButtonStyle::Prominent)
                        .content(text!("prominent"))),
                    button!(ButtonProps::new(|_|{}, always(false))
                        .style(ButtonStyle::Neutral)
                        .content(text!("neutral"))),
                    button!(ButtonProps::new(|_|{}, always(false))
                        .style(ButtonStyle::Unimportant)
                        .content(text!("unimportant")))
                ])),

            card!(static_list!(vec![
                text!("ButtonType::Elevated"),
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
                ])),
                card!(static_list!(vec![
                        text!("ButtonType::Text"),
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
                )),
                card!(static_list!(vec![
                        text!("ButtonType::Outlined"),
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
                )),
                card!(static_list!([
                    button!(
                            ButtonProps::new(clone!(counter => move |_| {
                                let v = *counter.lock_ref();
                                *counter.lock_mut() = v + 1;
                            }), always(false))
                            .content(dynamic_text!(map_ref! {
                                let value = counter.signal() => format!("Clicked {} times", value)
                            }))),
                    dynamic_text!(map_ref! {
                        let value = counter.signal() => format!("Button with dynamic content -  value is {}", value)
                    })
                ]))
            ])
    })
}
