use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::Mutable;

use dominator_material::components::{
    button, card, dynamic_text, text, ButtonProps, ButtonType, CardProps,
};
use dominator_material::utils::mixin::{mixin_id, no_mixin};

pub fn button_demo() -> Dom {
    let counter = Mutable::new(0);

    card(CardProps::new()
        .body(html!("div", {
            .class("demo-cards")
            .class("demo-card")
            .children(&mut[
                card(CardProps::new()
                    .header(html!("div", { .text("ButtonType::Contained") }))
                    .body(
                        button(ButtonProps::new()
                            .content(text("Click me!", no_mixin))
                            .on_click(|_| {
                                web_sys::window().unwrap().alert_with_message("You clicked?").unwrap();
                            }), mixin_id())), mixin_id()),
                card(CardProps::new()
                    .header(html!("div", { .text("ButtonType::Text") }))
                    .body(button(ButtonProps::new()
                        .content(text("Click me!", no_mixin))
                        .button_type(ButtonType::Text),mixin_id())), mixin_id()),
                card(CardProps::new()
                    .header(html!("div", { .text("ButtonType::Outlined") }))
                    .body(button(ButtonProps::new()
                        .content(text("Click me!", no_mixin))
                        .button_type(ButtonType::Outlined), mixin_id())), mixin_id()),
                card(CardProps::new()
                    .header(dynamic_text(map_ref! {
                        let value = counter.signal() => format!("Button with dynamic content -  value is {}", value)
                    }, no_mixin))
                    .body(
                        button(
                            ButtonProps::new()
                            .content_signal(map_ref! {
                                let value = counter.signal() => text(format!("Clicked {} times", value).as_str(), no_mixin)
                            })
                            .on_click(clone!(counter => move |_| {
                                let v = *counter.lock_ref();
                                *counter.lock_mut() = v + 1;
                            })),mixin_id())), mixin_id()),
            ])
        }),
    ), |v| v.class("demo-cards"))
}
