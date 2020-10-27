use dominator::{clone, Dom, events, html};
use wasm_bindgen::__rt::std::rc::Rc;

pub enum ButtonType {
    Contained,
    Outlined,
    Text
}

pub struct ButtonData {
    content: Option<Dom>,
    click_handler: Option<Rc<dyn Fn(events::Click)>>,
    button_type: ButtonType
}

pub struct Button {
    data: ButtonData
}

impl Button {
    pub fn new() -> Self {
        Button {
            data: ButtonData {
                content: None,
                click_handler: None,
                button_type: ButtonType::Contained
            }
        }
    }

    #[inline]
    pub fn on_click<F>(mut self: Self, handler: F) -> Self
        where F: Fn(events::Click) + 'static {
        self.data.click_handler = Some(Rc::new(handler));
        self
    }

    #[inline]
    pub fn text<T: Into<String>>(mut self: Self, text: T) -> Self {
        self.data.content = Some(html!("span", { .text(text.into().as_str()) }));
        self
    }

    #[inline]
    pub fn button_type(mut self, button_type: ButtonType) -> Self {
        self.data.button_type = button_type;
        self
    }

    #[inline]
    pub fn dom_content(mut self: Self, dom: Dom) -> Self {
        self.data.content = Some(dom);
        self
    }

    pub fn render(self: Self) -> Dom {
        button(Rc::new(self.data))
    }
}

#[inline]
fn button(button: Rc<ButtonData>) -> Dom {
    Dom::with_state(button, |button| {
        html!("button", {
            .class("dmat-button")
            .class( match button.button_type {
                ButtonType::Contained => "-contained",
                ButtonType::Outlined => "-outlined",
                ButtonType::Text => "-text",
            })
            .child(if button.content.is_some() {
                    Rc::get_mut(button).unwrap().content.take().unwrap()
                } else {
                html!("span")
            })
            .event(clone!(button => move |e: events::Click| {
                if let Some(handler) = &button.click_handler {
                    handler(e);
                }
            }))
        })
    })
}
