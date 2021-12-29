use dominator::{html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::MutableSignal;

use dominator_material::components::layouts::{app_bar, container, AppBarProps, AppBarType};
use dominator_material::components::{
    card, carousel, navigation_drawer, CardProps, CarouselProps, CarouselSource,
    NavigationDrawerEntry, NavigationDrawerProps, NavigationEntry,
};

use crate::components::navigation_drawer_demo::NavigationDrawerDemo;

pub fn app_bar_demo() -> Dom {
    card(
        CardProps::new().with_apply(|v| v.class("demo-card")).body(
            carousel(CarouselProps {
                source: AppBarCarousel::new(),
                apply: Some(Box::new(|d| d.class("demo-carousel"))),
                initial_view_index: Default::default(),
            })
            .0,
        ),
    )
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
                navigation_drawer(
                    NavigationDrawerProps::new()
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
                                    .main(container(
                                        html!("div", { .text(lipsum::lipsum(512).as_str())}),
                                    ))
                                    .fixed(),
                            ))
                        }),
                )
                .1
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
            ),
            _ => app_bar(
                AppBarProps::new()
                    .header(html!("h1", {
                        .class("app-bar-demo-header")
                        .text("Prominent fixed app bar")
                    }))
                    .main(NavigationDrawerDemo::static_drawers(true))
                    .bar_type(AppBarType::Prominent)
                    .fixed(),
            ),
        };

        html!("div", {
            .class("showcase-drawer-carousel-inner")
            .child(inner)
        })
    }

    fn total_count_signal(&self) -> MutableSignal<usize> {
        self.count.signal()
    }

    fn total_count(&self) -> usize {
        self.count.get()
    }
}
