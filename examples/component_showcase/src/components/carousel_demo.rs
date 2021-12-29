use std::rc::Rc;

use dominator::{clone, html, text_signal, Dom};
use futures_signals::signal::{Mutable, MutableSignal};
use futures_signals::signal_vec::{always, MutableSignalVec, MutableVec};

use dominator_material::components::layouts::Container;
use dominator_material::components::{
    button, card, carousel, static_list, text, ButtonProps, CardProps, CarouselProps,
    CarouselSource,
};

pub fn carousel_demo() -> Dom {
    card(CardProps::new().body(static_list(vec![
        carousel(CarouselProps {
            source: CarouselDemoSource::new(),
            apply: Some(Box::new(|d| d.class("demo-carousel"))),
            initial_view_index: Default::default(),
        }).0
    ])))
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
        Container::new(html!("div", { .text(format!("{}", index).as_str()) })).render()
    }

    fn total_count_signal(&self) -> MutableSignal<usize> {
        self.count.signal()
    }

    fn total_count(&self) -> usize {
        self.count.get()
    }
}
