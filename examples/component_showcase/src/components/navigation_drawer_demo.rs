use dominator::{clone, Dom, html};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;

use dominator_material::components::{
    Button, ButtonType, Card, NavigationDrawer, NavigationDrawerEntry, NavigationEntry,
};
use dominator_material::components::layouts::Container;

pub struct NavigationDrawerDemo {}

#[derive(Clone, PartialEq)]
enum ExampleViews {
    Main,
    Details,
    Other,
}

impl NavigationDrawerDemo {
    pub fn new() -> NavigationDrawerDemo {
        NavigationDrawerDemo {}
    }

    pub fn render(self) -> Dom {
        html!("div", {
            .children(&mut [
                Card::new()
                    .title("Static navigation drawer", None)
                    .body(html!("div", {
                        .class("navigation-drawer-demo")
                        .child(Self::static_drawers())
                    }))
                    .render(),
                Card::new()
                    .title("Modal navigation drawer", None)
                    .body(html!("div", {
                        .class("navigation-drawer-demo")
                        .child(Self::modal_drawers())
                    }))
                    .render()
            ])
        })
    }

    fn static_drawers() -> Dom {
        Self::make_drawer()
            .show_toggle_controls(true)
            .render()
    }

    fn modal_drawers() -> Dom {
        Self::make_drawer()
            .show_toggle_controls(true)
            .modal(true)
            .render()
    }

    fn make_drawer() -> NavigationDrawer<ExampleViews> {
        NavigationDrawer::new()
            .initial_selected(ExampleViews::Main)
            .title_view_generator(|v, _| match v {
                Some(ExampleViews::Main) => Some(html!("span", { .text("Main view") })),
                Some(ExampleViews::Details) => Some(html!("span", { .text("Details") })),
                Some(ExampleViews::Other) => Some(html!("span", { .text("Other view") })),
                _ => Some(html!("span", { .text("Some view") })),
            })
            .entries(vec![
                NavigationDrawerEntry::Item(NavigationEntry {
                    text: "Main".into(),
                    id: ExampleViews::Main,
                }),
                NavigationDrawerEntry::Item(NavigationEntry {
                    text: "Details".into(),
                    id: ExampleViews::Details,
                }),
                NavigationDrawerEntry::Item(NavigationEntry {
                    text: "Other".into(),
                    id: ExampleViews::Other,
                }),
            ])
            .main_view_generator(move |v, drawer_handle| {
                Some(
                    Container::new(match v {
                        Some(ExampleViews::Main) => html!("span", {
                            .text("Main view")
                        }),
                        Some(ExampleViews::Details) => html!("span", { .text("Details") }),
                        Some(ExampleViews::Other) => html!("span", { .text("Other view") }),
                        _ => html!("span", { .text("Some view") }),
                    })
                        .render(),
                )
            })
    }
}
