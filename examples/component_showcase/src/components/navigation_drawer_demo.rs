use dominator::{clone, Dom, html};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;

use dominator_material::components::{Button, Card, NavigationDrawer, NavigationDrawerEntry, NavigationEntry, ButtonType};
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
        let (drawer_handle, drawer_dom) = NavigationDrawer::new()
            .initial_active(ExampleViews::Main)
            .title_view_generator(|v, _| {
                match v {
                    Some(ExampleViews::Main) => Some(html!("span", { .text("Main view") })),
                    Some(ExampleViews::Details) => Some(html!("span", { .text("Details") })),
                    Some(ExampleViews::Other) => Some(html!("span", { .text("Other view") })),
                    _ => Some(html!("span", { .text("Some view") })),
                }
            })
            .main_view_generator(move |v, drawer_handle| {
                Some(Container::new(
                    match v {
                            Some(ExampleViews::Main) => html!("span", {
                                .text("Main view")
                                .child(Button::new()
                                    .on_click(clone!(drawer_handle => move |_| {
                                        let val = drawer_handle.expanded.get();
                                        drawer_handle.expanded.set(!val);
                                    }))
                                    .text("Toggle")
                                    .button_type(ButtonType::Text)
                                    .render())
                            }),
                            Some(ExampleViews::Details) => html!("span", { .text("Details") }),
                            Some(ExampleViews::Other) => html!("span", { .text("Other view") }),
                            _ => html!("span", { .text("Some view") }),
                        }).render())
                    })
            .render_with_handle();

        drawer_handle.set_entries(vec![
            NavigationDrawerEntry::Item(NavigationEntry{ text: "Main".into(), id: ExampleViews::Main}),
            NavigationDrawerEntry::Item(NavigationEntry{ text: "Details".into(), id: ExampleViews::Details}),
            NavigationDrawerEntry::Item(NavigationEntry{ text: "Other".into(), id: ExampleViews::Other}),
        ]);

        Card::new()
            .body(html!("div", {
                .class("navigation-drawer-demo")
                .child(drawer_dom)
            }))
            .render()
    }
}
