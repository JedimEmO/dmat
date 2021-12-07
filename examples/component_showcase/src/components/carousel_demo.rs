use dominator::{html, Dom};
use futures_signals::signal::{Mutable, MutableSignal};

use dominator_material::components::layouts::Container;
use dominator_material::components::{card, CardProps, Carousel, CarouselSource};

pub struct CarouselDemo {}

impl CarouselDemo {
    pub fn new() -> CarouselDemo {
        CarouselDemo {}
    }

    pub fn render(self) -> Dom {
        card(CardProps::new().with_body(
            Carousel::new(CarouselDemoSource::new()).render_apply(|d, _| d.class("demo-carousel")),
        ))
    }
}

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
        Container::new(html!("div", { .text(format!("{}", index).as_str()) })).render()
    }

    fn total_count_signal(&self) -> MutableSignal<usize> {
        self.count.signal()
    }

    fn total_count(&self) -> usize {
        self.count.get()
    }
}
