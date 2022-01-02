use dominator::{html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::MutableSignal;

use crate::components::navigation_drawer_demo::static_drawers;
use dominator_material::components::layouts::{app_bar, container, AppBarProps, AppBarType};
use dominator_material::components::{
    card, carousel, navigation_drawer, CardProps, CarouselProps, CarouselSource,
    NavigationDrawerEntry, NavigationDrawerProps, NavigationEntry,
};

pub fn app_bar_demo() -> Dom {
    card(
        CardProps::new().body(
            carousel(CarouselProps {
                source: AppBarCarousel::new(),
                initial_view_index: Default::default(),
            })
            .0
            .apply(|d| d.class("demo-carousel"))
            .into_dom(),
        ),
    )
    .apply(|v| v.class("demo-card"))
    .into_dom()
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
            0 => navigation_drawer(
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
                    .title_view_generator(|_, _| Some(html!("div", {.text("Outer modal drawer")})))
                    .modal(true)
                    .main_view_generator(|_, _| {
                        Some(
                            app_bar(
                                AppBarProps::new()
                                    .header(html!("div", {
                                        .class("app-bar-demo-header")
                                    }))
                                    .main(container(
                                        html!("div", { .text(lipsum::lipsum(512).as_str())}),
                                    ))
                                    .fixed(),
                            )
                            .into_dom(),
                        )
                    }),
            )
            .0
            .class("demo-drawer-with-app-bar")
            .into_dom(),
            1 => app_bar(
                AppBarProps::new()
                    .header(html!("h1", {
                        .class("app-bar-demo-header")
                        .text("Normal unfixed app bar")
                    }))
                    .main(html!("div", {
                        .text(lipsum::lipsum(1024).as_str())
                    })),
            )
            .into_dom(),
            _ => app_bar(
                AppBarProps::new()
                    .header(html!("h1", {
                        .class("app-bar-demo-header")
                        .text("Prominent fixed app bar")
                    }))
                    .main(static_drawers(true))
                    .bar_type(AppBarType::Prominent)
                    .fixed(),
            )
            .into_dom(),
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
