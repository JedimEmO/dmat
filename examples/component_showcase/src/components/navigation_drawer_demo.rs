use dominator::{Dom, html};

use dominator_material::components::{Card, NavigationDrawer};

pub struct NavigationDrawerDemo {}

impl NavigationDrawerDemo {
    pub fn new() -> NavigationDrawerDemo {
        NavigationDrawerDemo {}
    }

    pub fn render(self) -> Dom {
        Card::new()
            .body(html!("div", {
                .class("navigation-drawer-demo")
                .child(NavigationDrawer::new()
                        .main(Card::new().render()).render())
            }))
            .render()
    }
}
