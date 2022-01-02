use dominator::{html, Dom};
use futures_signals::signal::{Mutable, MutableSignal};

use dominator_material::components::layouts::container;
use dominator_material::components::{
    card, carousel, CardProps, CarouselProps, CarouselSource,
};

pub fn carousel_demo() -> Dom {
    card(
        CardProps::new().body(
            carousel(CarouselProps {
                source: CarouselDemoSource::new(),
                initial_view_index: Default::default(),
            })
            .0
            .apply(|d| d.class("demo-carousel"))
            .into_dom(),
        ),
    )
    .into_dom()
}

#[derive(Clone)]
struct CarouselDemoSource {
    count: Mutable<usize>,
}

impl CarouselDemoSource {
    pub fn new() -> CarouselDemoSource {
        CarouselDemoSource {
            count: Mutable::new(3),
        }
    }
}

impl CarouselSource for CarouselDemoSource {
    fn get_entry(&self, index: usize) -> Dom {
        container(html!("div", {
            .text(format!("{}", index).as_str())
        }))
        .style("width", "30rem")
        .style("height", "30rem")
        .into_dom()
    }

    fn total_count_signal(&self) -> MutableSignal<usize> {
        self.count.signal()
    }

    fn total_count(&self) -> usize {
        self.count.get()
    }
}
