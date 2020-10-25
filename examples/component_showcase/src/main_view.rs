use dominator::{clone, Dom, html};

use dominator_material::components::{Card, Tabs, Tab};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;

pub struct MainView {}

impl MainView {
    pub fn build() -> MainView {
        MainView {}
    }

    pub fn dom(self) -> Dom {
        let active_tab = Mutable::new(0);

        html!("div", {
            .class("main-view")
            .children(&mut [
                Tabs::build()
                .active_tab_id(Some(0))
                .static_tabs(vec![
                    Tab {
                        label: "Card".into(),
                        id: 0
                    },
                    Tab {
                        label: "List".into(),
                        id: 1
                    },
                    Tab {
                        label: "Tabs".into(),
                        id: 2
                    },
                    Tab {
                        label: "Data Table".into(),
                        id: 3
                    },
                    Tab {
                        label: "Input".into(),
                        id: 4
                    },
                ])
                .on_tab_change(clone!(active_tab => move |id| {
                    if let Some(id) = id {
                        active_tab.set_neq(id);
                    }
                })).dom(),
                Card::build(clone!(active_tab => move || {
                    html!("div", {
                        .child_signal(active_tab.signal().map(|tab_id| {
                            Some(html!("span", {
                                .text(format!("{}", tab_id).as_str())
                            }))
                        }))
                    })
                }))
                .dom()
            ])
        })
    }
}
