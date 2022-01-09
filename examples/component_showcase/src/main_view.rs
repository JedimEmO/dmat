use dominator::{clone, html, Dom};
use dominator_material::components::layouts::{app_bar, AppBarProps};
use dominator_material::components::{Tab, TabContent};
use dominator_material::utils::mixin::mixin_id;
use futures_signals::signal::SignalExt;

use futures_signals::signal_vec::always;
use std::cell::RefCell;
use std::rc::Rc;

use crate::components::app_bar_demo::app_bar_demo;
use crate::components::button_demo::button_demo;
use crate::components::card_demo::card_demo;
use crate::components::carousel_demo::carousel_demo;
use crate::components::data_table_demo::data_table_demo;
use crate::components::input_demo::input_demo;
use crate::components::list_demo::list_demo;
use crate::components::navigation_drawer_demo::navigation_drawers_demo;
use crate::route::DemoRoute;

pub fn main_view() -> Dom {
    let active_tab = DemoRoute::signal();

    app_bar(
        AppBarProps::new()
            .header(tabs!(
                |id| DemoRoute::signal().map(clone!(id => move |v| v == id)),
                always(main_view_tabs()),
                Some(Rc::new(RefCell::new(|new_tab| {
                    DemoRoute::goto(new_tab)
                })))
            ))
            .main_signal(active_tab.map(|tab_id| match tab_id {
                DemoRoute::AppBar => app_bar_demo(),
                DemoRoute::Button => button_demo(),
                DemoRoute::List => list_demo(),
                DemoRoute::Carousel => carousel_demo(),
                DemoRoute::Card => card_demo(),
                DemoRoute::DataTable => data_table_demo(),
                DemoRoute::Input => input_demo(),
                DemoRoute::NavigationDrawer => navigation_drawers_demo(),
                _ => html!("div"),
            }))
            .fixed(),
        mixin_id(),
    )
}

fn main_view_tabs() -> Vec<Tab<DemoRoute>> {
    vec![
        Tab {
            content: TabContent::Label("App Bar".into()),
            id: DemoRoute::AppBar,
        },
        Tab {
            content: TabContent::Label("Button".into()),
            id: DemoRoute::Button,
        },
        Tab {
            content: TabContent::Label("Carousel".into()),
            id: DemoRoute::Carousel,
        },
        Tab {
            content: TabContent::Label("Card".into()),
            id: DemoRoute::Card,
        },
        Tab {
            content: TabContent::Label("List".into()),
            id: DemoRoute::List,
        },
        Tab {
            content: TabContent::Label("Tabs".into()),
            id: DemoRoute::Tabs,
        },
        Tab {
            content: TabContent::Label("Data Table".into()),
            id: DemoRoute::DataTable,
        },
        Tab {
            content: TabContent::Label("Input".into()),
            id: DemoRoute::Input,
        },
        Tab {
            content: TabContent::Label("Navigation Drawer".into()),
            id: DemoRoute::NavigationDrawer,
        },
    ]
}
