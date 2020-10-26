use dominator::{Dom, events, html};
use wasm_bindgen::__rt::std::rc::Rc;

enum ButtonContent {
    Text(String),
    Dom(Box<dyn Fn() -> Dom>),
}

pub struct Button {
    content: ButtonContent,
    click_handler: Option<Rc<dyn Fn(events::Click)>>,
}

impl Button {
    pub fn new() -> Rc<Self> {
        Rc::new(Button {
            content: ButtonContent::Text("Missing content".into()),
            click_handler: None,
        })
    }

    pub fn on_click<F>(mut self: Rc<Self>, handler: F) -> Rc<Self>
        where F: Fn(events::Click) + 'static {
        Rc::get_mut(&mut self).unwrap().click_handler = Some(Rc::new(handler));
        self
    }

    pub fn text(mut self: Rc<Self>, text: String) -> Rc<Self> {
        Rc::get_mut(&mut self).unwrap().content = ButtonContent::Text(text);
        self
    }

    pub fn dom_generator<F: 'static>(mut self: Rc<Self>, dom_generator: F) -> Rc<Self>
        where F: Fn() -> Dom {
        Rc::get_mut(&mut self).unwrap().content = ButtonContent::Dom(Box::new(dom_generator));
        self
    }

    pub fn render(self: Rc<Self>) -> Dom {
        button(self)
    }
}

#[inline]
fn button(button: Rc<Button>) -> Dom {
    Dom::with_state(button, |button| {
        let on_click = match &button.click_handler {
            Some(handler) => Some(handler.clone()),
            _ => None
        };

        html!("button", {
            .class("dmat-button")
            .child(match &button.content {
                ButtonContent::Text(txt) => {
                    html!("span", { .text(txt.as_str()) })
                }
                ButtonContent::Dom(dom_generator) => dom_generator()
            })
            .event(move |e: events::Click| {
                if let Some(handler) = &on_click {
                    handler(e);
                }
            })
        })
    })
}
