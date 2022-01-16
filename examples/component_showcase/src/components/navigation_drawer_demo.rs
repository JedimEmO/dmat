use dominator::{html, Dom};
use futures_signals::signal::{Mutable, ReadOnlyMutable};
use futures_signals::signal_vec::MutableVec;

use dominator_material::components::{
    CardProps, DrawerWidth, NavigationDrawerEntry, NavigationDrawerProps,
};

#[derive(Clone, Copy, PartialEq)]
enum ExampleViews {
    Main,
    Details,
    Other,
}

#[inline]
pub fn navigation_drawers_demo() -> Dom {
    container!(|d| d.children(&mut [
        card!(
            CardProps::new()
                .with_title("Static navigation drawer", None)
                .body(html!("div", {
                    .class("navigation-drawer-demo")
                    .child(static_drawers(true, DrawerWidth::Full))
                })),
            |d| d.class("drawer-demo-card").style("height", "350px")
        ),
        card!(
            CardProps::new()
                .with_title("Modal navigation drawer", None)
                .body(html!("div", {
                    .class("navigation-drawer-demo")
                    .child(modal_drawers())
                })),
            |d| d.class("drawer-demo-card").style("height", "350px")
        ),
        card!(
            CardProps::new()
                .with_title("Static navigation drawer without toggle controls", None)
                .body(html!("div", {
                    .class("navigation-drawer-demo")
                    .child(static_drawers(false, DrawerWidth::Full))
                })),
            |d| d.class("drawer-demo-card").style("height", "350px")
        ),
        card!(
            CardProps::new()
                .with_title(
                    "Static narrow navigation drawer without toggle controls",
                    None
                )
                .body(html!("div", {
                    .class("navigation-drawer-demo")
                    .child(static_drawers(false, DrawerWidth::Narrow))
                })),
            |d| d.class("drawer-demo-card").style("height", "350px")
        ),
    ]))
}

pub fn static_drawers(toggle: bool, width: DrawerWidth) -> Dom {
    let props = NavigationDrawerProps {
        entries: MutableVec::new_with_values(demo_items()),
        main_view_generator: |_| None,
        header_view_generator: None,
        item_renderer: |item, width| render_example_view(item, width),
        show_toggle_controls: false,
        is_modal: false,
        expanded: Mutable::new(true),
        current_active: Default::default(),
        width: Mutable::new(width).read_only(),
    }
    .show_toggle_controls(toggle);

    navigation_drawer!(props).0
}

fn modal_drawers() -> Dom {
    let props = NavigationDrawerProps {
        entries: MutableVec::new_with_values(demo_items()),
        main_view_generator: |_| None,
        header_view_generator: None,
        item_renderer: |item, width| render_example_view(item, width),
        show_toggle_controls: true,
        is_modal: true,
        expanded: Mutable::new(true),
        current_active: Default::default(),
        width: Mutable::new(DrawerWidth::Full).read_only(),
    }
    .header_view_generator(|item, width| Some(html!("Examples")));

    navigation_drawer!(props).0
}

fn render_example_view(view: ExampleViews, width: DrawerWidth) -> Dom {
    if width == DrawerWidth::Narrow {
        return match view {
            ExampleViews::Main => text!("M"),
            ExampleViews::Details => text!("D"),
            ExampleViews::Other => text!("O"),
        };
    }

    match view {
        ExampleViews::Main => text!("Main"),
        ExampleViews::Details => text!("Details"),
        ExampleViews::Other => text!("Other"),
    }
}

fn demo_items() -> Vec<NavigationDrawerEntry<ExampleViews>> {
    vec![
        NavigationDrawerEntry::Item(ExampleViews::Main),
        NavigationDrawerEntry::Item(ExampleViews::Details),
        NavigationDrawerEntry::Item(ExampleViews::Other),
    ]
}

// fn make_drawer() -> NavigationDrawerProps<ExampleViews> {
//     NavigationDrawerProps::new()
//         .initial_selected(ExampleViews::Main)
//         .title_view_generator(|v, _| match v {
//             Some(ExampleViews::Main) => Some(html!("span", { .text("Main view") })),
//             Some(ExampleViews::Details) => Some(html!("span", { .text("Details") })),
//             Some(ExampleViews::Other) => Some(html!("span", { .text("Other view") })),
//             _ => Some(html!("span", { .text("Some view") })),
//         })
//         .entries(vec![
//             NavigationDrawerEntry::Item(NavigationEntry {
//                 text: "Main".into(),
//                 id: ExampleViews::Main,
//             }),
//             NavigationDrawerEntry::Item(NavigationEntry {
//                 text: "Details".into(),
//                 id: ExampleViews::Details,
//             }),
//             NavigationDrawerEntry::Item(NavigationEntry {
//                 text: "Other".into(),
//                 id: ExampleViews::Other,
//             }),
//         ])
//         .main_view_generator(move |v, _handle| {
//             Some(container!(|d| {
//                 d.child(match v {
//                     Some(ExampleViews::Main) => html!("span", {
//                         .text("Main view")
//                     }),
//                     Some(ExampleViews::Details) => html!("span", { .text("Details") }),
//                     Some(ExampleViews::Other) => html!("span", { .text("Other view") }),
//                     _ => html!("span", { .text("Some view") }),
//                 })
//             }))
//         })
// }
