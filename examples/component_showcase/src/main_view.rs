use dominator::{html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::always;

use dominator_material::components::layouts::{app_bar, AppBarProps};
use dominator_material::components::{Tab, TabContent};
use dominator_material::utils::mixin::mixin_id;

use crate::components::app_bar_demo::app_bar_demo;
use crate::components::button_demo::button_demo;
use crate::components::card_demo::card_demo;
use crate::components::carousel_demo::carousel_demo;
use crate::components::data_table_demo::data_table_demo;
use crate::components::input_demo::input_demo;
use crate::components::list_demo::list_demo;
use crate::components::navigation_drawer_demo::navigation_drawers_demo;

#[derive(Copy, Clone, PartialEq, Debug)]
enum DemoTabs {
    AppBar,
    Button,
    List,
    Carousel,
    Card,
    Tabs,
    DataTable,
    Input,
    NavigationDrawer,
}

pub fn main_view() -> Dom {
    let active_tab = Mutable::new(DemoTabs::Input);

    app_bar(
        AppBarProps::new()
            .header(tabs!(active_tab.clone(), always(main_view_tabs()), None))
            .main_signal(active_tab.signal().map(|tab_id| match tab_id {
                DemoTabs::AppBar => app_bar_demo(),
                DemoTabs::Button => button_demo(),
                DemoTabs::List => list_demo(),
                DemoTabs::Carousel => carousel_demo(),
                DemoTabs::Card => card_demo(),
                DemoTabs::DataTable => data_table_demo(),
                DemoTabs::Input => input_demo(),
                DemoTabs::NavigationDrawer => navigation_drawers_demo(),
                _ => html!("div"),
            }))
            .fixed(),
        mixin_id(),
    )
}

fn main_view_tabs() -> Vec<Tab<DemoTabs>> {
    vec![
        Tab {
            content: TabContent::Label("App Bar".into()),
            id: DemoTabs::AppBar,
        },
        Tab {
            content: TabContent::Label("Button".into()),
            id: DemoTabs::Button,
        },
        Tab {
            content: TabContent::Label("Carousel".into()),
            id: DemoTabs::Carousel,
        },
        Tab {
            content: TabContent::Label("Card".into()),
            id: DemoTabs::Card,
        },
        Tab {
            content: TabContent::Label("List".into()),
            id: DemoTabs::List,
        },
        Tab {
            content: TabContent::Label("Tabs".into()),
            id: DemoTabs::Tabs,
        },
        Tab {
            content: TabContent::Label("Data Table".into()),
            id: DemoTabs::DataTable,
        },
        Tab {
            content: TabContent::Label("Input".into()),
            id: DemoTabs::Input,
        },
        Tab {
            content: TabContent::Label("Navigation Drawer".into()),
            id: DemoTabs::NavigationDrawer,
        },
    ]
}
