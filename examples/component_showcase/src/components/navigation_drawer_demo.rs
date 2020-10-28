use dominator::{Dom, html};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;

use dominator_material::components::{Card, NavigationDrawer, NavigationDrawerEntry};

pub struct NavigationDrawerDemo {}

impl NavigationDrawerDemo {
    pub fn new() -> NavigationDrawerDemo {
        NavigationDrawerDemo {}
    }

    pub fn render(self) -> Dom {
        let f = web_sys::DocumentFragment::new();
        let current_view = Mutable::new(0);

        Card::new()
            .body(html!("div", {
                .class("navigation-drawer-demo")
                .child(NavigationDrawer::new()
                    .connect(|v| {
                        v.set_entries(vec![
                            NavigationDrawerEntry::Item,
                            NavigationDrawerEntry::Item,
                            NavigationDrawerEntry::Item,
                            NavigationDrawerEntry::Item,
                        ])
                    })
                    .main(current_view.signal_cloned().map(|v| {
                        Some(html!("span", { .text("Main view") }))
                    }))
                    .render())
            }))
            .render()
    }
}
