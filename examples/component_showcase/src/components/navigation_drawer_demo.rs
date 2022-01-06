use dominator::{html, Dom};

use dominator_material::components::layouts::container;
use dominator_material::components::{
    CardProps, NavigationDrawerEntry, NavigationDrawerProps, NavigationEntry,
};
use dominator_material::utils::mixin::mixin_id;

#[derive(Clone, PartialEq)]
enum ExampleViews {
    Main,
    Details,
    Other,
}

#[inline]
pub fn navigation_drawers_demo() -> Dom {
    card!(
        CardProps::new().body(static_list!(vec![
            card!(
                CardProps::new()
                    .with_title("Static navigation drawer", None)
                    .body(html!("div", {
                        .class("navigation-drawer-demo")
                        .child(static_drawers(true))
                    })),
                |d| d.class("drawer-demo-card")
            ),
            card!(
                CardProps::new()
                    .with_title("Modal navigation drawer", None)
                    .body(html!("div", {
                        .class("navigation-drawer-demo")
                        .child(modal_drawers())
                    })),
                |d| d.class("drawer-demo-card")
            ),
            card!(
                CardProps::new()
                    .with_title("Static navigation drawer without toggle controls", None)
                    .body(html!("div", {
                        .class("navigation-drawer-demo")
                        .child(static_drawers(false))
                    })),
                |d| d.class("drawer-demo-card")
            ),
        ])),
        |v| v.class("demo-card")
    )
}

pub fn static_drawers(toggle: bool) -> Dom {
    navigation_drawer!(make_drawer().show_toggle_controls(toggle)).0
}

fn modal_drawers() -> Dom {
    navigation_drawer!(make_drawer().show_toggle_controls(true).modal(true)).0
}

fn make_drawer() -> NavigationDrawerProps<ExampleViews> {
    NavigationDrawerProps::new()
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
            Some(container(
                match v {
                    Some(ExampleViews::Main) => html!("span", {
                        .text("Main view")
                    }),
                    Some(ExampleViews::Details) => html!("span", { .text("Details") }),
                    Some(ExampleViews::Other) => html!("span", { .text("Other view") }),
                    _ => html!("span", { .text("Some view") }),
                },
                mixin_id(),
            ))
        })
}
