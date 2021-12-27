use dominator::{html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::always;
use wasm_bindgen::__rt::std::rc::Rc;

use dominator_material::components::layouts::{app_bar, AppBarProps};
use dominator_material::components::{layouts::Container, tabs, Tab, TabContent};

use crate::components::app_bar_demo::AppBarDemo;
use crate::components::button_demo::ButtonDemo;
use crate::components::card_demo::CardDemo;
use crate::components::carousel_demo::CarouselDemo;
use crate::components::data_table_demo::{data_table_demo, DataTableDemo};
use crate::components::input_demo::InputDemo;
use crate::components::list_demo::list_demo;
use crate::components::navigation_drawer_demo::NavigationDrawerDemo;

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

pub struct MainView {
    active_tab: Mutable<DemoTabs>,
}

impl MainView {
    pub fn new() -> Rc<MainView> {
        Rc::new(MainView {
            active_tab: Mutable::new(DemoTabs::AppBar),
        })
    }

    pub fn render(self: Rc<Self>) -> Dom {
        let active_tab = self.active_tab.clone();

        Dom::with_state(self, |main_view| {
            app_bar(
                AppBarProps::new()
                    .header(tabs(
                        active_tab.clone(),
                        always(vec![
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
                        ]),
                        None,
                    ))
                    .main_signal(
                        main_view
                            .active_tab
                            .signal()
                            .map(|tab_id| match tab_id {
                                DemoTabs::AppBar => AppBarDemo::new().render(),
                                DemoTabs::Button => ButtonDemo::new().render(),
                                DemoTabs::List => list_demo(),
                                DemoTabs::Carousel => CarouselDemo::new().render(),
                                DemoTabs::Card => CardDemo::new().render(),
                                DemoTabs::DataTable => data_table_demo(),
                                DemoTabs::Input => InputDemo::new().render(),
                                DemoTabs::NavigationDrawer => NavigationDrawerDemo::new().render(),
                                _ => html!("div"),
                            })
                            .map(|v| Container::new(v).render()),
                    )
                    .fixed(),
            )
        })
    }
}
