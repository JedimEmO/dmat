use dominator::{html, Dom};
use futures_signals::signal::always;
use futures_signals::signal::Mutable;
use futures_signals::signal::MutableSignal;
use lipsum::lipsum;

use dominator_material::components::layouts::{app_bar, AppBarProps, AppBarType};
use dominator_material::components::{
    CardProps, CarouselProps, CarouselSource, DrawerWidth, NavigationDrawerProps,
};

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
            count: Mutable::new(2),
        }
    }
}

impl CarouselSource for AppBarCarousel {
    fn get_entry(&self, index: usize) -> Dom {
        let inner = match index {
            0 => app_bar(
                AppBarProps::new()
                    .header(html!("h1", {
                        .class("app-bar-demo-header")
                        .text("Normal unfixed app bar")
                    }))
                    .main(html!("div", {
                        .text(lipsum::lipsum(1024).as_str())
                    })),
                |d| d.style("height", "500px"),
            ),
            _ => app_bar(
                AppBarProps::new()
                    .header(html!("h1", {
                        .class("app-bar-demo-header")
                        .text("Prominent fixed app bar")
                    }))
                    .main(
                        navigation_drawer!(NavigationDrawerProps {
                            main_content: container!(|d| d.child(html!("div", {
                                .text(lipsum(200).as_str())
                            }))),
                            drawer_content: static_list!(vec![
                                html!("div", {.text("Inbox")}),
                                html!("div", {.text("Spam")})
                            ]),
                            width: DrawerWidth::Full,
                            modal: false,
                            retracts: false,
                            visible_signal: always(true),
                            with_scrim: false
                        })
                        .0,
                    )
                    .bar_type(AppBarType::Prominent)
                    .fixed(),
                |d| d.style("height", "500px"),
            ),
        };

        container!(|d| d
            .child(inner)
            .style("padding", "16px")
            .style("position", "relative")
            .style("margin", "auto")
            .style("width", "500px")
            .style("height", "500px"))
    }

    fn total_count_signal(&self) -> MutableSignal<usize> {
        self.count.signal()
    }

    fn total_count(&self) -> usize {
        self.count.get()
    }
}
