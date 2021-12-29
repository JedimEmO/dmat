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
    let images = carousel(CarouselProps {
        source: CarouselDemoSource::new(),
        apply: Some(Box::new(|d| d.class("demo-carousel"))),
        initial_view_index: Default::default(),
    });

    let images_ctrl = images.1;

    card(CardProps::new().body(static_list(vec![
        images.0,
        button(ButtonProps::new().content(text("test")).on_click(
            clone!(images_ctrl => move |_| {
                images_ctrl.goto_index(5).unwrap();
            }),
        )),
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
