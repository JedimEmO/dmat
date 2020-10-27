use dominator::{clone, Dom, events, html};
use wasm_bindgen::__rt::std::rc::Rc;

pub struct Button {
    content: Option<Dom>,
    click_handler: Option<Rc<dyn Fn(events::Click)>>,
}

impl Button {
    pub fn new() -> Self {
        Button {
            content: None,
            click_handler: None,
        }
    }

    pub fn on_click<F>(mut self: Self, handler: F) -> Self
        where F: Fn(events::Click) + 'static {
        self.click_handler = Some(Rc::new(handler));
        self
    }

    pub fn text<T: Into<String>>(mut self: Self, text: T) -> Self {
        self.content = Some(html!("span", { .text(text.into().as_str()) }));
        self
    }

    pub fn dom_content(mut self: Self, dom: Dom) -> Self {
        self.content = Some(dom);
        self
    }

    pub fn render(self: Self) -> Dom {
        button(Rc::new(self))
    }
}

#[inline]
fn button(button: Rc<Button>) -> Dom {
    Dom::with_state(button, |button| {
        html!("button", {
            .class("dmat-button")
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
