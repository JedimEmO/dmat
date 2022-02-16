use dominator::{html, Dom};
use futures_signals::signal::{Mutable, MutableSignal};

use dominator_material::components::{CarouselProps, CarouselSource};

pub fn carousel_demo() -> Dom {
    card!(
        carousel!(
            CarouselProps {
                source: CarouselDemoSource::new(),
                initial_view_index: Default::default(),
            },
            |d| d.class("demo-carousel")
        )
        .0
    )
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
        container!(|d| {
            d.child(html!("div", {
                .text(format!("{}", index).as_str())
            }))
            .style("width", "30rem")
            .style("height", "30rem")
        })
    }

    fn total_count_signal(&self) -> MutableSignal<usize> {
        self.count.signal()
    }

    fn total_count(&self) -> usize {
        self.count.get()
    }
}
