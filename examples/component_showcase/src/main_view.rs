use dominator::{clone, html, Dom};
use dominator_material::components::layouts::{app_bar, AppBarProps};
use dominator_material::components::TabsProps;
use dominator_material::utils::mixin::mixin_id;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::MutableVec;
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
use crate::components::sheet_demo::sheet_demo;
use crate::route::DemoRoute;

pub fn main_view() -> Dom {
    let active_tab = DemoRoute::signal();

    app_bar(
        AppBarProps::new()
            .header(tabs!(TabsProps {
                tab_fn: main_view_tabs,
                active_tab_signal_factory: |id| DemoRoute::signal()
                    .map(clone!(id => move |v| v == id)),
                tabs_list: MutableVec::new_with_values(main_view_tab_list().into()).signal_vec(),
                on_tab_change: Some(Rc::new(RefCell::new(|new_tab| {
                    DemoRoute::goto(new_tab)
                })))
            }))
            .main_signal(active_tab.map(|tab_id| match tab_id {
                DemoRoute::AppBar => app_bar_demo(),
                DemoRoute::Button => button_demo(),
                DemoRoute::List => list_demo(),
                DemoRoute::Carousel => carousel_demo(),
                DemoRoute::Card => card_demo(),
                DemoRoute::DataTable => data_table_demo(),
                DemoRoute::Input => input_demo(),
                DemoRoute::NavigationDrawer => navigation_drawers_demo(),
                DemoRoute::Sheet => sheet_demo(),
                _ => html!("div"),
            }))
            .fixed(),
        mixin_id(),
    )
}

fn main_view_tab_list() -> [DemoRoute; 10] {
    [
        DemoRoute::AppBar,
        DemoRoute::List,
        DemoRoute::Button,
        DemoRoute::Tabs,
        DemoRoute::DataTable,
        DemoRoute::Carousel,
        DemoRoute::Input,
        DemoRoute::NavigationDrawer,
        DemoRoute::Card,
        DemoRoute::Sheet,
    ]
}
fn main_view_tabs(tab_id: DemoRoute) -> Dom {
    match tab_id {
        DemoRoute::Tabs => text!("Tabs"),
        DemoRoute::DataTable => text!("Data Table"),
        DemoRoute::AppBar => text!("App Bar"),
        DemoRoute::Card => text!("Card"),
        DemoRoute::List => text!("List"),
        DemoRoute::NavigationDrawer => text!("Navigation Drawer"),
        DemoRoute::Input => text!("Input"),
        DemoRoute::Carousel => text!("Carousel"),
        DemoRoute::Button => text!("Button"),
        DemoRoute::Sheet => text!("Sheet"),
    }
}
