use dominator::{html, Dom};
use futures_signals::signal::always;
use lipsum::lipsum;

use crate::components::navigation_drawer_demo::mock_view_select;
use dominator_material::components::layouts::{app_bar, AppBarProps, AppBarType};
use dominator_material::components::{CardProps, DrawerWidth, NavigationDrawerProps};

pub fn app_bar_demo() -> Dom {
    static_list!([container!(|d| d.children(&mut [
        card!(CardProps::new().body(normal_unfixed_demo())),
        card!(CardProps::new().body(prominent_fixed_demo())),
    ]))])
}

fn normal_unfixed_demo() -> Dom {
    app_bar(
        AppBarProps::new()
            .header(html!("h1", {
                .class("app-bar-demo-header")
                .text("Normal unfixed app bar")
            }))
            .main(html!("div", {
                .text(lipsum::lipsum(1024).as_str())
            })),
        |d| d.style("height", "500px"),
    )
}

fn prominent_fixed_demo() -> Dom {
    app_bar(
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
                    drawer_content: mock_view_select(),
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
    )
}
