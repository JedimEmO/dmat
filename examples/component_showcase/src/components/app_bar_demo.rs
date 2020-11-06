use dominator::{html, Dom};
use futures_signals::signal::always;
use futures_signals::signal::SignalExt;

use dominator_material::components::layouts::{AppBarType, Container};
use dominator_material::components::{
    layouts::AppBar, Card, List, NavigationDrawer, NavigationDrawerEntry, NavigationEntry,
};
use dominator_material::utils::renderable_child::IntoRenderableChild;

use crate::components::navigation_drawer_demo::NavigationDrawerDemo;

pub struct AppBarDemo {}

impl AppBarDemo {
    pub fn new() -> AppBarDemo {
        AppBarDemo {}
    }

    pub fn render(self) -> Dom {
        let app_bar_standard = AppBar::new()
            .header(
                html!("h1", {
                    .class("app-bar-demo-header")
                    .text("Normal unfixed app bar")
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
                html!("h1", {
                    .class("app-bar-demo-header")
                    .text("Prominent fixed app bar")
                })
                .into_renderable_child(),
            )
            .main(always(true).map(|_| Some(NavigationDrawerDemo::static_drawers(true))))
            .bar_type(AppBarType::Prominent)
            .fixed()
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
            .title_view_generator(|_, _| Some(html!("div", {.text("Outer modal drawer")})))
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
                        .fixed()
                        .render(),
                )
            })
            .render();

        Card::new()
            .apply(|v| v.class("demo-card"))
            .body(List::new_static(vec![
                html!("div", {
                    .class("app-bar-demo")
                    .child(drawer_with_app_bar)
                }),
                html!("div", {
                    .class("app-bar-demo")
                    .child(app_bar_standard)
                }),
                html!("div", {
                    .class("app-bar-demo")
                    .child(app_bar_with_drawer)
                }),
            ]))
            .render()
    }
}
