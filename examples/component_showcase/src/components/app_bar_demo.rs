use dominator::{html, Dom};
use dominator_material::components::{layouts::AppBar, Card};
use dominator_material::utils::renderable_child::IntoRenderableChild;

use futures_signals::signal::always;
use futures_signals::signal::Signal;
use futures_signals::signal::SignalExt;

pub struct AppBarDemo {}

impl AppBarDemo {
    pub fn new() -> AppBarDemo {
        AppBarDemo {}
    }

    pub fn render(self) -> Dom {
        let main = always(true);
        let app_bar_card = Card::new()
            .apply(|v| v.class("demo-card").class("app-bar-demo"))
            .body(
                AppBar::new()
                    .header(
                        html!("div", {
                            .text("hei")
                        })
                        .into_renderable_child(),
                    )
                    .main(main.map(|v| {
                        Some(html!("div", {
                            .text(lipsum::lipsum(1024).as_str())
                        }))
                    }))
                    .render(),
            )
            .render();

        Card::new().body(app_bar_card).render()
    }
}
