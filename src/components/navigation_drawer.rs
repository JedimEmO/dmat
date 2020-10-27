use dominator::{Dom, html};

pub struct NavigationDrawer {
    main: Option<Dom>
}

impl NavigationDrawer {
    pub fn new() -> NavigationDrawer {
        NavigationDrawer { main: None }
    }

    #[inline]
    pub fn main(mut self, dom: Dom) -> Self {
        self.main = Some(html!("div", {
            .class("dmat-navigation-drawer-main")
            .class("dmat-surface")
            .child(dom)
        }));
        self
    }

    pub fn render(self) -> Dom {
        html!("div", {
            .class("dmat-navigation-drawer-container")
            .children(vec![
                Some(html!("div", {
                    .class("dmat-navigation-drawer")
                })),
                self.main
            ].into_iter().filter_map(|v| v))
        })
    }
}
