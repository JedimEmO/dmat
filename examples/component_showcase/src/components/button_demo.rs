use dominator::{Dom, html};
use wasm_bindgen::__rt::std::rc::Rc;

use dominator_material::components::{Button, List};

pub struct ButtonDemo {}

impl ButtonDemo {
    pub fn new() -> Rc<ButtonDemo> {
        Rc::new(ButtonDemo {})
    }

    pub fn render(self: Rc<Self>) -> Dom {
        html!("div", {
            .child(List::new_static(vec![
                Button::new()
                    .text("Click me!")
                    .on_click(|_| {
                        web_sys::window().unwrap().alert_with_message("You clicked?").unwrap();
                    })
                    .render(),
                Button::new()
                    .dom_content(
                        html!("ul", {
                            .children(&mut [
                                html!("li",  { .text("This button contains")}),
                                html!("li",  { .text("a list")})
                            ])
                        })
                    )
                    .render()
            ]))
        })
    }
}
