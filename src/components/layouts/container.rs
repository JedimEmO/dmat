use dominator::{Dom, html};

pub struct Container {
    child: Dom
}

impl Container {
    pub fn new(child: Dom) -> Container {
        Container { child }
    }

    pub fn render(self) -> Dom {
        html!("div", {
            .class("dmat-container")
            .child(self.child)
        })
    }
}
