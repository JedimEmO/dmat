use dominator::{html, Dom};

use dominator_material::components::layouts::Container;
use dominator_material::components::{
    Card, List, NavigationDrawer, NavigationDrawerEntry, NavigationEntry,
};

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
        Card::new()
            .apply(|v| v.class("demo-card"))
            .body(List::new_static(vec![
                Card::new()
                    .title("Static navigation drawer", None)
                    .body(html!("div", {
                        .class("navigation-drawer-demo")
                        .child(Self::static_drawers(true))
                    }))
                    .render(),
                Card::new()
                    .title("Modal navigation drawer", None)
                    .body(html!("div", {
                        .class("navigation-drawer-demo")
                        .child(Self::modal_drawers())
                    }))
                    .render(),
                Card::new()
                    .title("Static navigation drawer without toggle controls", None)
                    .body(html!("div", {
                        .class("navigation-drawer-demo")
                        .child(Self::static_drawers(false))
                    }))
                    .render(),
            ]))
            .render()
    }

    pub fn static_drawers(toggle: bool) -> Dom {
        Self::make_drawer().show_toggle_controls(toggle).render()
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
            .main_view_generator(move |v, _handle| {
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
