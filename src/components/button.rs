use dominator::{clone, events, html, Dom};
use wasm_bindgen::__rt::std::rc::Rc;

pub enum ButtonType {
    Contained,
    Outlined,
    Text,
}

pub struct ButtonProps {
    pub content: Box<dyn Fn() -> Dom>,
    pub click_handler: Option<Rc<dyn Fn(events::Click)>>,
    pub button_type: ButtonType,
}

impl ButtonProps {
    pub fn new() -> Self {
        Self {
            content: Box::new(|| html!("span")),
            click_handler: None,
            button_type: ButtonType::Contained,
        }
    }

    #[inline]
    pub fn on_click<F>(mut self: Self, handler: F) -> Self
    where
        F: Fn(events::Click) + 'static,
    {
        self.click_handler = Some(Rc::new(handler));
        self
    }

    #[inline]
    pub fn text<T: Into<String>>(mut self: Self, text: T) -> Self {
        let text = text.into().clone();
        self.content =
            Box::new(clone!(text => move || html!("span", { .text(text.clone().as_str()) })));
        self
    }

    #[inline]
    pub fn button_type(mut self, button_type: ButtonType) -> Self {
        self.button_type = button_type;
        self
    }

    #[inline]
    pub fn dom_content<TDom>(mut self: Self, dom: TDom) -> Self
    where
        TDom: Fn() -> Dom + 'static,
    {
        self.content = Box::new(dom);
        self
    }
}

pub fn button(props: ButtonProps) -> Dom {
    Dom::with_state(props, |button| {
        let click_handler = button.click_handler.clone();

        html!("button", {
            .class("dmat-button")
            .class( match button.button_type {
                ButtonType::Contained => "-contained",
                ButtonType::Outlined => "-outlined",
                ButtonType::Text => "-text",
            })
            .child((*button.content)())
            .apply_if(button.click_handler.is_some(), |dom| {
                dom.event(clone!(click_handler => move |e: events::Click| {
                    (&click_handler.as_ref().unwrap())(e);
                }))
            })
        })
    })
}
