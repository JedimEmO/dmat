use dominator::{html, Dom};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};

use dmat_components::components::ListEntry;
use dmat_components::components::*;

use crate::components::app_bar_demo::app_bar_demo;
use crate::components::button_demo::button_demo;
use crate::components::card_demo::card_demo;
use crate::components::carousel_demo::carousel_demo;
use crate::components::dock_overlay_demo::dock_overlay_demo;
use crate::components::input_demo::input_demo;
use crate::components::list_demo::list_demo;
use crate::components::navigation_drawer_demo::navigation_drawers_demo;
use crate::components::sheet_demo::sheet_demo;
use crate::components::table_demo::table_demo;

use crate::route::{DemoRoute, ExampleAppRoute};

pub fn component_demo_view(current_route: DemoRoute) -> Dom {
    let component_list = component_demo_list(current_route);

    navigation_drawer!({
        .drawer_content(Some(component_list))
        .main_content(Some(component_demo(current_route)))
    })
}

fn component_demo_list(current_value: DemoRoute) -> Dom {
    let entries_raw = MutableVec::new_with_values(vec![
        DemoRoute::AppBar,
        DemoRoute::Button,
        DemoRoute::Card,
        DemoRoute::Carousel,
        DemoRoute::Table,
        DemoRoute::DockOverlay,
        DemoRoute::Input,
        DemoRoute::List,
        DemoRoute::NavigationDrawer,
        DemoRoute::Sheet,
    ]);

    let entries = entries_raw.signal_vec().map(|entry| ListEntry {
        before: None,
        content: render_demo_label(entry),
        after: None,
    });

    interactive_list!({
        .items_signal_vec(entries)
        .selected_indexes(vec![entries_raw.lock_ref().iter().position(|entry| *entry == current_value).unwrap_or(0)])
        .on_item_selected(move |entry| {
            let new_component = entries_raw.lock_ref()[entry];
            ExampleAppRoute::goto(ExampleAppRoute::Components(new_component))
        })
    })
}

fn component_demo(component: DemoRoute) -> Dom {
    html!("div", {
        .attr("id", "demo-view")
        .child(match component {
                DemoRoute::AppBar => app_bar_demo(),
                DemoRoute::Button => button_demo(),
                DemoRoute::List => list_demo(),
                DemoRoute::Carousel => carousel_demo(),
                DemoRoute::Card => card_demo(),
                DemoRoute::Table => table_demo(),
                DemoRoute::Input => input_demo(),
                DemoRoute::NavigationDrawer => navigation_drawers_demo(),
                DemoRoute::Sheet => sheet_demo(),
                DemoRoute::DockOverlay => dock_overlay_demo(),
                _ => html!("div"),
            })
    })
}

fn render_demo_label(tab_id: DemoRoute) -> Dom {
    match tab_id {
        DemoRoute::Tabs => html!("span", {.text("Tabs") }),
        DemoRoute::AppBar => html!("span", {.text("App Bar") }),
        DemoRoute::Button => html!("span", {.text("Button") }),
        DemoRoute::List => html!("span", {.text("List") }),
        DemoRoute::Carousel => html!("span", {.text("Carousel") }),
        DemoRoute::Card => html!("span", {.text("Card") }),
        DemoRoute::Table => html!("span", {.text("Table") }),
        DemoRoute::Input => html!("span", {.text("Input") }),
        DemoRoute::NavigationDrawer => html!("span", {.text("Navigation Drawer") }),
        DemoRoute::Sheet => html!("span", {.text("Sheet") }),
        DemoRoute::DockOverlay => html!("span", {.text("Dock Overlay") }),
    }
}
