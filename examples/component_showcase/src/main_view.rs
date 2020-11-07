use dominator::{clone, html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use wasm_bindgen::__rt::std::rc::Rc;

use dominator_material::components::layouts::AppBar;
use dominator_material::components::{layouts::Container, Tab, Tabs};

use crate::components::app_bar_demo::AppBarDemo;
use crate::components::button_demo::ButtonDemo;
use crate::components::card_demo::CardDemo;
use crate::components::data_table_demo::DataTableDemo;
use crate::components::input_demo::InputDemo;
use crate::components::list_demo::ListDemo;
use crate::components::navigation_drawer_demo::NavigationDrawerDemo;
use dominator_material::utils::renderable_child::IntoRenderableChild;

#[derive(Clone, PartialEq)]
enum DemoTabs {
    AppBar,
    Button,
    List,
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
            active_tab: Mutable::new(DemoTabs::Card),
        })
    }

    pub fn render(self: Rc<Self>) -> Dom {
        Dom::with_state(self, |main_view| {
            AppBar::new()
                .header(
                    Tabs::new()
                        .initial_active_tab_id(Some(DemoTabs::AppBar))
                        .on_tab_change(clone!(main_view => move |id| {
                            if let Some(id) = id {
                                main_view.active_tab.set_neq(id);
                            }
                        }))
                        .build_static(vec![
                            Tab {
                                label: "App Bar".into(),
                                id: DemoTabs::AppBar,
                            },
                            Tab {
                                label: "Button".into(),
                                id: DemoTabs::Button,
                            },
                            Tab {
                                label: "Card".into(),
                                id: DemoTabs::Card,
                            },
                            Tab {
                                label: "List".into(),
                                id: DemoTabs::List,
                            },
                            Tab {
                                label: "Tabs".into(),
                                id: DemoTabs::Tabs,
                            },
                            Tab {
                                label: "Data Table".into(),
                                id: DemoTabs::DataTable,
                            },
                            Tab {
                                label: "Input".into(),
                                id: DemoTabs::Input,
                            },
                            Tab {
                                label: "Navigation Drawer".into(),
                                id: DemoTabs::NavigationDrawer,
                            },
                        ])
                        .into_renderable_child(),
                )
                .main(
                    main_view
                        .active_tab
                        .signal_cloned()
                        .map(|tab_id| match tab_id {
                            DemoTabs::AppBar => Some(AppBarDemo::new().render()),
                            DemoTabs::Button => Some(ButtonDemo::new().render()),
                            DemoTabs::List => Some(ListDemo::new().render()),
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
