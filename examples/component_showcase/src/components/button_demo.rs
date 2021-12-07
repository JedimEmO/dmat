use std::iter::once;

use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::Mutable;
use wasm_bindgen::__rt::std::rc::Rc;

use dominator_material::components::{
    button, card, dynamic_text, text, ButtonProps, ButtonType, CardProps,
};
use dominator_material::utils::component_signal::once_cmp;

pub struct ButtonDemo {}

impl ButtonDemo {
    pub fn new() -> Rc<ButtonDemo> {
        Rc::new(ButtonDemo {})
    }

    pub fn render(self: Rc<Self>) -> Dom {
        let counter = Mutable::new(0);

        card(CardProps::new()
            .with_apply(|v| v.class("demo-cards"))
            .with_body(html!("div", {
                .class("demo-cards")
                .class("demo-card")
                .children(&mut[
                    card(CardProps::new()
                        .with_header(html!("div", { .text("ButtonType::Contained") }))
                        .with_body(button(ButtonProps::new(once_cmp(text("Click me!")))
                            .on_click(|_| {
                                web_sys::window().unwrap().alert_with_message("You clicked?").unwrap();
                            })))),
                    card(CardProps::new()
                        .with_header(html!("div", { .text("ButtonType::Text") }))
                        .with_body(button(ButtonProps::new(once_cmp(text("Click me!")))
                            .button_type(ButtonType::Text)))),
                    card(CardProps::new()
                        .with_header(html!("div", { .text("ButtonType::Outlined") }))
                        .with_body(button(ButtonProps::new(once_cmp(text("Click me!")))
                            .button_type(ButtonType::Outlined)))),
                    card(CardProps::new()
                        .with_header(dynamic_text(map_ref! {
                            let value = counter.signal() => format!("Button with dynamic content -  value is {}", value)
                        }))
                        .with_body(
                            button(
                                ButtonProps::new(once(map_ref! {
                                    let value = counter.signal() => Some(text(format!("Clicked {} times", value)))
                                }))
                                .on_click(clone!(counter => move |_| {
                                    let v = *counter.lock_ref();
                                    *counter.lock_mut() = v + 1;
                                })))))
                ])
            })))
    }
}
