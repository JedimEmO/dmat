use dominator::{html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::MutableSignal;

use dominator_material::components::layouts::{app_bar, AppBarProps, AppBarType, Container};
use dominator_material::components::{
    card, navigation_drawer, CardProps, Carousel, CarouselSource, NavigationDrawerEntry,
    NavigationDrawerProps, NavigationEntry,
};

use crate::components::navigation_drawer_demo::NavigationDrawerDemo;

pub struct AppBarDemo {}

impl AppBarDemo {
    pub fn new() -> AppBarDemo {
        AppBarDemo {}
    }

    pub fn render(self) -> Dom {
        card(CardProps::new().with_apply(|v| v.class("demo-card")).body(
            Carousel::new(AppBarCarousel::new()).render_apply(|d, _| d.class("demo-carousel")),
        ))
    }
}

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
                                    .main(
                                        Container::new(
                                            html!("div", { .text(lipsum::lipsum(512).as_str())}),
                                        )
                                        .render(),
                                    )
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
