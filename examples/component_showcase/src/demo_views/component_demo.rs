use dominator::{clone, html, Dom};
use futures::Stream;
use futures_signals::signal::always;
use futures_signals::signal_vec::{MutableVec, SignalVecExt};

use dmat_components::components::ListEntry;
use dmat_components::components::{DrawerWidth, InteractiveListProps, NavigationDrawerProps};
use dmat_components::utils::mixin::stream_handler_mixin;

use crate::components::app_bar_demo::app_bar_demo;
use crate::components::button_demo::button_demo;
use crate::components::card_demo::card_demo;
use crate::components::carousel_demo::carousel_demo;
use crate::components::data_table_demo::data_table_demo;
use crate::components::dock_overlay_demo::dock_overlay_demo;
use crate::components::input_demo::input_demo;
use crate::components::list_demo::list_demo;
use crate::components::navigation_drawer_demo::navigation_drawers_demo;
use crate::components::sheet_demo::sheet_demo;

use crate::route::{DemoRoute, ExampleAppRoute};

pub fn component_demo_view(current_route: DemoRoute) -> Dom {
    let (component_list, component_change_stream) = component_demo_list(current_route);

    let app_navigation_props = NavigationDrawerProps {
        visible_signal: always(true),
        with_scrim: false,
        width: DrawerWidth::Full,
        retracts: false,
        modal: false,
        drawer_content: component_list,
        main_content: component_demo(current_route, component_change_stream),
    };

    navigation_drawer!(app_navigation_props).0
}

fn component_demo_list(current_value: DemoRoute) -> (Dom, impl Stream<Item = Option<DemoRoute>>) {
    let entries = MutableVec::new_with_values(vec![
        DemoRoute::AppBar,
        DemoRoute::Button,
        DemoRoute::Card,
        DemoRoute::Carousel,
        DemoRoute::DataTable,
        DemoRoute::DockOverlay,
        DemoRoute::Input,
        DemoRoute::List,
        DemoRoute::NavigationDrawer,
        DemoRoute::Sheet,
    ]);

    let entries = entries
        .signal_vec()
        .map(clone!(current_value => move |entry| ListEntry {
            before: None,
            content: render_demo_label(entry),
            after: None,
            selected_signal: Box::new(always(current_value == entry)),
            item_value: entry,
        }));

    let list_props = InteractiveListProps { items: entries };
    let (list_dom, list_out) = interactive_list!(list_props);

    (list_dom, list_out.item_select_stream)
}

fn component_demo<TStream: Stream<Item = Option<DemoRoute>> + Unpin + 'static>(
    component: DemoRoute,
    component_change_stream: TStream,
) -> Dom {
    html!("div", {
        .apply(stream_handler_mixin(component_change_stream, |new_component| {
            if let Some(c) = new_component {
                ExampleAppRoute::goto(ExampleAppRoute::Components(c))
            } else {
                ExampleAppRoute::goto(ExampleAppRoute::About)
            }
        }))
        .attribute("id", "demo-view")
        .child(match component {
                DemoRoute::AppBar => app_bar_demo(),
                DemoRoute::Button => button_demo(),
                DemoRoute::List => list_demo(),
                DemoRoute::Carousel => carousel_demo(),
                DemoRoute::Card => card_demo(),
                DemoRoute::DataTable => data_table_demo(),
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
        DemoRoute::Tabs => text!("Tabs"),
        DemoRoute::DataTable => text!("Data Table"),
        DemoRoute::AppBar => text!("App Bar"),
        DemoRoute::Card => text!("Card"),
        DemoRoute::DockOverlay => text!("Dock Overlay"),
        DemoRoute::List => text!("List"),
        DemoRoute::NavigationDrawer => text!("Navigation Drawer"),
        DemoRoute::Input => text!("Input"),
        DemoRoute::Carousel => text!("Carousel"),
        DemoRoute::Button => text!("Button"),
        DemoRoute::Sheet => text!("Sheet"),
    }
}
