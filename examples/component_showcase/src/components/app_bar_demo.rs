use dominator::{html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::MutableSignal;

use crate::components::navigation_drawer_demo::static_drawers;
use dominator_material::components::layouts::{app_bar, AppBarProps, AppBarType};
use dominator_material::components::{
    CardProps, CarouselProps, CarouselSource, NavigationDrawerEntry, NavigationDrawerProps,
    NavigationEntry,
};
use dominator_material::utils::mixin::mixin_id;

pub fn app_bar_demo() -> Dom {
    card!(CardProps::new().body(
        carousel!(
            CarouselProps {
                source: AppBarCarousel::new(),
                initial_view_index: Default::default(),
            },
            |d| d.class("demo-carousel")
        )
        .0,
    ))
}

#[derive(Clone)]
struct AppBarCarousel {
    count: Mutable<usize>,
}

impl AppBarCarousel {
    fn new() -> AppBarCarousel {
        AppBarCarousel {
            count: Mutable::new(3),
        }
    }
}

impl CarouselSource for AppBarCarousel {
    fn get_entry(&self, index: usize) -> Dom {
        let inner = match index {
            0 => {
                navigation_drawer!(
                    NavigationDrawerProps::new()
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
                        .title_view_generator(|_, _| {
                            Some(html!("div", {.text("Outer modal drawer")}))
                        })
                        .modal(true)
                        .main_view_generator(|_, _| {
                            Some(app_bar(
                                AppBarProps::new()
                                    .header(html!("div", {
                                        .class("app-bar-demo-header")
                                    }))
                                    .main(container!(|d| d.child(
                                        html!("div", { .text(lipsum::lipsum(512).as_str())})
                                    )))
                                    .fixed(),
                                mixin_id(),
                            ))
                        }),
                    |d| d.class("demo-drawer-with-app-bar")
                )
                .0
            }
            1 => app_bar(
                AppBarProps::new()
                    .header(html!("h1", {
                        .class("app-bar-demo-header")
                        .text("Normal unfixed app bar")
                    }))
                    .main(html!("div", {
                        .text(lipsum::lipsum(1024).as_str())
                    })),
                mixin_id(),
            ),
            _ => app_bar(
                AppBarProps::new()
                    .header(html!("h1", {
                        .class("app-bar-demo-header")
                        .text("Prominent fixed app bar")
                    }))
                    .main(static_drawers(true))
                    .bar_type(AppBarType::Prominent)
                    .fixed(),
                mixin_id(),
            ),
        };

        container!(|d| d
            .child(inner)
            .style("width", "500px")
            .style("padding", "16px")
            .style("height", "500px"))
    }

    fn total_count_signal(&self) -> MutableSignal<usize> {
        self.count.signal()
    }

    fn total_count(&self) -> usize {
        self.count.get()
    }
}
