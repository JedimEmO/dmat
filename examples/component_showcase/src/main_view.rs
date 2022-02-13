use std::cell::RefCell;
use std::rc::Rc;

use dominator::{clone, html, with_node, Dom};
use futures::Stream;
use futures_signals::signal::{always, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};

use dominator_material::components::layouts::{app_bar, AppBarProps};
use dominator_material::components::{
    CardProps, DrawerWidth, InteractiveListProps, ListEntry, NavigationDrawerProps, TabsProps,
};
use dominator_material::utils::mixin::{with_id, with_stream_handler};

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

pub fn main_view() -> Dom {
    let active_tab = ExampleAppRoute::signal();

    app_bar(
        AppBarProps::new()
            .header(html!("div", {
                .children(&mut [
                    html!("h1", {
                       .text("Dominator Material")
                    }),
                    tabs!(TabsProps {
                        tab_fn: render_top_level_tabs,
                        active_tab_signal_factory: |id| ExampleAppRoute::signal()
                            .map(clone!(id => move |v| v == id)),
                        tabs_list: MutableVec::new_with_values(vec![
                            ExampleAppRoute::About,
                            ExampleAppRoute::Components(DemoRoute::AppBar)
                        ])
                        .signal_vec(),
                        on_tab_change: Some(Rc::new(RefCell::new(|new_tab| {
                            ExampleAppRoute::goto(new_tab)
                        })))
                    })
                ])
            }))
            .main(main_app_view(active_tab))
            .fixed(),
        with_id("dmat-example-app"),
    )
}

fn main_app_view<S: Signal<Item = ExampleAppRoute> + 'static>(active_route: S) -> Dom {
    html!("div", {
        .child_signal(active_route.map(|route| {
            match route {
                ExampleAppRoute::About => Some(about_view()),
                ExampleAppRoute::Components(c) => Some(component_demo_view(c)),
            }
        }))
    })
}

fn component_demo_view(current_route: DemoRoute) -> Dom {
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
        .apply(with_stream_handler(component_change_stream, |new_component| {
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

fn render_top_level_tabs(route: ExampleAppRoute) -> Dom {
    match route {
        ExampleAppRoute::About => text!("About"),
        ExampleAppRoute::Components(_) => text!("Components"),
    }
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

fn about_view() -> Dom {
    container!(|d| {
        d.child(card!(CardProps::new()
        .with_title("Dominator Material", Some("A performance first FRP component library"))
            .body(html!("div", {
                .children(& mut [
                    html!("p", {
                        .with_node!(e => {
                            .apply(|d| {
                                e.set_inner_html("Dominator Material is a component library made for the <a href=\"https://github.com/Pauan/rust-dominator\" target=\"_blank\">dominator</a> library. It is written in rust, and compiles to webassembly for optimal runtime and binary size performance.");
                                d
                            })
                        })
                    }),
                    html!("p", {
                        .text("It is a functional reactive programming library.")
                    })
                ])
            }))
        ))
    })
}
