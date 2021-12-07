use dominator::{html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::always;
use wasm_bindgen::__rt::std::rc::Rc;

use dominator_material::components::layouts::AppBar;
use dominator_material::components::{layouts::Container, tabs, Tab, TabContent};
use dominator_material::utils::renderable_child::IntoRenderableChild;

use crate::components::app_bar_demo::AppBarDemo;
use crate::components::button_demo::ButtonDemo;
use crate::components::card_demo::CardDemo;
use crate::components::carousel_demo::CarouselDemo;
use crate::components::data_table_demo::DataTableDemo;
use crate::components::input_demo::InputDemo;
use crate::components::list_demo::list_demo;
use crate::components::navigation_drawer_demo::NavigationDrawerDemo;

#[derive(Clone, PartialEq, Debug)]
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
            AppBar::new()
                .header(
                    tabs(
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
                    )
                    .into_renderable_child(),
                )
                .main(
                    main_view
                        .active_tab
                        .signal_cloned()
                        .map(|tab_id| match tab_id {
                            DemoTabs::AppBar => Some(AppBarDemo::new().render()),
                            DemoTabs::Button => Some(ButtonDemo::new().render()),
                            DemoTabs::List => Some(list_demo()),
                            DemoTabs::Carousel => Some(CarouselDemo::new().render()),
                            DemoTabs::Card => Some(CardDemo::new().render()),
                            DemoTabs::DataTable => Some(DataTableDemo::new().render()),
                            DemoTabs::Input => Some(InputDemo::new().render()),
                            DemoTabs::NavigationDrawer => {
                                Some(NavigationDrawerDemo::new().render())
                            }
                            _ => Some(html!("div")),
                        })
                        .map(|v| Some(Container::new(v.unwrap()).render())),
                )
                .fixed()
                .render()
        })
    }
}
