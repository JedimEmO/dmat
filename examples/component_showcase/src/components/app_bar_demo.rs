use dominator::{html, Dom};
use lipsum::lipsum;

use dmat_components::components::layouts::*;
use dmat_components::components::*;

use crate::components::navigation_drawer_demo::mock_view_select;

pub fn app_bar_demo() -> Dom {
    list!({
        .rows([
            container!({
                .children([
                    card!({
                        .child(normal_unfixed_demo())
                    }),
                    card!({
                        .child(prominent_fixed_demo())
                    })
                ])
            })
        ])
    })
}

fn normal_unfixed_demo() -> Dom {
    app_bar!({
        .header(Some(html!("h1", {
            .class("app-bar-demo-header")
            .text("Normal unfixed app bar")
        })))
        .main(Some(html!("div", {
            .text(lipsum::lipsum(1024).as_str())
        })))
        .apply(|d| d.style("height", "500px"))
    })
}

fn prominent_fixed_demo() -> Dom {
    let drawer = navigation_drawer!({
        .main_content(Some(container!({
            .children([html!("div", {
                .text(lipsum(1024).as_str())
            })])
        })))
        .drawer_content(Some(mock_view_select()))
    });

    app_bar!({
        .header(Some(html!("h1", {
            .class("app-bar-demo-header")
            .text("Prominent fixed app bar")
        })))
        .main(Some(drawer))
        .app_bar_type(AppBarType::Prominent)
        .fixed(true)
        .apply(|d| d.style("height", "500px"))
    })
}
