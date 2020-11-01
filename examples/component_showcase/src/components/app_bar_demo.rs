use dominator::{html, Dom};
use futures_signals::signal::always;
use futures_signals::signal::SignalExt;

use dominator_material::components::{
    layouts::AppBar, Card, List, NavigationDrawer, NavigationDrawerEntry, NavigationEntry,
};
use dominator_material::utils::renderable_child::IntoRenderableChild;

use crate::components::navigation_drawer_demo::NavigationDrawerDemo;
use dominator_material::components::layouts::Container;

pub struct AppBarDemo {}

impl AppBarDemo {
    pub fn new() -> AppBarDemo {
        AppBarDemo {}
    }

    pub fn render(self) -> Dom {
        let app_bar_standard = AppBar::new()
            .header(
                html!("div", {
                    .class("app-bar-demo-header")
                    .text("hei")
                })
                .into_renderable_child(),
            )
            .main(always(true).map(|_| {
                Some(html!("div", {
                    .text(lipsum::lipsum(1024).as_str())
                }))
            }))
            .render();

        let app_bar_with_drawer = AppBar::new()
            .header(
                html!("div", {
                    .class("app-bar-demo-header")
                    .text("hei")
                })
                .into_renderable_child(),
            )
            .main(always(true).map(|_| Some(NavigationDrawerDemo::static_drawers(true))))
            .render();

        let drawer_with_app_bar = NavigationDrawer::new()
            .apply(|_, dom| dom.class("demo-drawer-with-app-bar"))
            .show_toggle_controls(true)
            .expanded(true)
            .initial_selected(0)
            .entries(vec![
                NavigationDrawerEntry::Item(NavigationEntry {
                    id: 0,
                    text: "Inbox".to_string(),
                }),
                NavigationDrawerEntry::Item(NavigationEntry {
                    id: 1,
                    text: "Spam".to_string(),
                }),
            ])
            .modal(true)
            .main_view_generator(|_, _| {
                Some(
                    AppBar::new()
                        .header(
                            html!("div", {
                                .class("app-bar-demo-header")
                            })
                            .into_renderable_child(),
                        )
                        .main(
                            Container::new(html!("div", { .text(lipsum::lipsum(512).as_str())}))
                                .render()
                                .into_renderable_child(),
                        )
                        .render(),
                )
            })
            .render();

        let app_bar_card = Card::new()
            .apply(|v| v.class("demo-card"))
            .body(List::new_static(vec![
                Card::new()
                    .apply(|v| v.class("app-bar-demo"))
                    .body(app_bar_standard)
                    .title("Standard", None)
                    .render(),
                Card::new()
                    .apply(|v| v.class("app-bar-demo"))
                    .body(app_bar_with_drawer)
                    .title("Standard", None)
                    .render(),
                Card::new()
                    .apply(|v| v.class("app-bar-demo"))
                    .body(drawer_with_app_bar)
                    .title("Drawer with app bar", None)
                    .render(),
            ]))
            .render();

        Card::new().body(app_bar_card).render()
    }
}
