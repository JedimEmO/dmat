use dominator::{clone, html, Dom};

use dominator_material::components::{ButtonProps, ButtonType, CardProps};

use dominator_material::utils::mixin::mixin_id;
use futures_signals::map_ref;
use futures_signals::signal::Mutable;
pub fn button_demo() -> Dom {
    let counter = Mutable::new(0);

    card!(CardProps::new()
        .body(html!("div", {
            .class("demo-cards")
            .class("demo-card")
            .children(&mut[
                card!(CardProps::new()
                    .header(html!("div", { .text("ButtonType::Contained") }))
                    .body(
                        button!(ButtonProps::new()
                            .content(text!("Click me!"))
                            .on_click(|_| {
                                web_sys::window().unwrap().alert_with_message("You clicked?").unwrap();
                            })))),
                card!(CardProps::new()
                    .header(html!("div", { .text("ButtonType::Text") }))
                    .body(button!(ButtonProps::new()
                        .content(text!("Click me!"))
                        .button_type(ButtonType::Text),mixin_id()))),
                card!(CardProps::new()
                    .header(html!("div", { .text("ButtonType::Outlined") }))
                    .body(button!(ButtonProps::new()
                        .content(text!("Click me!"))
                        .button_type(ButtonType::Outlined)))),
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
        }),
    ), |v| v.class("demo-cards"))
}
