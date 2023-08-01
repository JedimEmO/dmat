use dmat_components::components::layouts::*;
use dominator::{html, Dom};
use futures_signals::signal::{Mutable, MutableSignal};

use dmat_components::components::*;

pub fn carousel_demo() -> Dom {
    card!({.child(carousel!(
            CarouselProps {
                source: CarouselDemoSource::new(),
                initial_view_index: Default::default(),
            },
            |d| d.class("demo-carousel")
        )
        .0)})
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
    fn get_entry(&self, _: usize) -> Dom {
        container!({
            .apply(|d| d.child(html!("img", {
                .attr("src", "images/shapes.svg")
                .attr("width", "60%")
                .attr("height", "100%")
                .attr("alt", "shapes!")
            })).class("demo-carousel-item"))
        })
    }

    fn total_count_signal(&self) -> MutableSignal<usize> {
        self.count.signal()
    }

    fn total_count(&self) -> usize {
        self.count.get()
    }
}
