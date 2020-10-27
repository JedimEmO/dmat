use dominator::{clone, Dom, html};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use wasm_bindgen::__rt::std::rc::Rc;

use dominator_material::components::{Card, Tab, Tabs};

use crate::components::button_demo::ButtonDemo;
use crate::components::list_demo::ListDemo;
use crate::components::card_demo::CardDemo;

#[derive(Clone, PartialEq)]
enum DemoTabs {
    Button,
    List,
    Card,
    Tabs,
    DataTable,
    Input,
}

pub struct MainView {
    active_tab: Mutable<DemoTabs>
}

impl MainView {
    pub fn new() -> Rc<MainView> {
        Rc::new(MainView { active_tab: Mutable::new(DemoTabs::Card) })
    }

    pub fn render(self: Rc<Self>) -> Dom {
        Dom::with_state(self, |main_view| {
            html!("div", {
                .class("main-view")
                .children(&mut [
                    Tabs::new()
                    .initial_active_tab_id(Some(DemoTabs::Card))
                    .on_tab_change(clone!(main_view => move |id| {
                        if let Some(id) = id {
                            main_view.active_tab.set_neq(id);
                        }
                    }))
                    .build_static(vec![
                        Tab {
                            label: "Button".into(),
                            id: DemoTabs::Button
                        },
                        Tab {
                            label: "Card".into(),
                            id: DemoTabs::Card
                        },
                        Tab {
                            label: "List".into(),
                            id: DemoTabs::List
                        },
                        Tab {
                            label: "Tabs".into(),
                            id: DemoTabs::Tabs
                        },
                        Tab {
                            label: "Data Table".into(),
                            id: DemoTabs::DataTable
                        },
                        Tab {
                            label: "Input".into(),
                            id: DemoTabs::Input
                        },
                    ]),
                    Card::new()
                        .body(html!("div", {
                            .child_signal(main_view.active_tab.signal_cloned().map(|tab_id| {
                                match tab_id {
                                    DemoTabs::Button => Some(ButtonDemo::new().render()),
                                    DemoTabs::List => Some(ListDemo::new().render()),
                                    DemoTabs::Card => Some(CardDemo::new().render()),
                                    _ => Some(html!("div"))
                                }
                            }))
                        }))
                        .render()
                ])
            })
        })
    }
}
