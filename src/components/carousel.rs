use crate::futures_signals::signal::SignalExt;
use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::internal::Map2;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, MutableSignal, Signal};
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub trait CarouselSource {
    fn get_entry(&self, index: usize) -> Dom;
    fn total_count_signal(&self) -> MutableSignal<usize>;
    fn total_count(&self) -> usize;
}

#[derive(Clone, Eq, PartialEq)]
pub enum OutgoingItemDirection {
    Left,
    Right,
}

#[derive(Clone)]
pub struct OutgoingItem {
    pub index: usize,
    pub direction: OutgoingItemDirection,
}

pub struct Carousel<T: CarouselSource + 'static> {
    pub current_item_index: Mutable<usize>,
    pub outgoing_item: Mutable<Option<OutgoingItem>>,
    current_active_child_element: Mutable<usize>,
    source: Rc<T>,
}

impl<T: CarouselSource> Carousel<T> {
    /// This produces a signal which will yield true if the child at `child_index`
    /// is considered leaving in `direction`
    fn child_leave_signal(
        &self,
        child_index: usize,
        direction: OutgoingItemDirection,
    ) -> impl Signal<Item = bool> {
        map_ref!(
            let outgoing = self.outgoing_item.signal_cloned(),
            let active_child_element = self.current_active_child_element.signal() => move {
                if *active_child_element == child_index {
                    return false;
                }

                if let Some(outgoing)= outgoing {
                    if outgoing.direction != direction {
                        return false;
                    }

                    return true
                }

                false
            }
        )
    }

    fn hidden_signal(&self, index: usize) -> impl Signal<Item = bool> {
        map_ref!(
            let transitioning = self.outgoing_item.signal_cloned(),
            let active = self.current_active_child_element.signal() => move {
                transitioning.is_none() && *active != index
            }
        )
    }

    fn child_signal(&self, index: usize) -> impl Signal<Item = Option<Dom>> {
        let source = self.source.clone();
        let transition = self.outgoing_item.clone();
        map_ref!(
            let current = self.current_item_index.signal(),
            let active = self.current_active_child_element.signal() => move {
                if *active == index {
                    return Some(source.get_entry(*current));
                }
                None
            }
        )
        .filter_map(clone!(transition => move |v| {
            if transition.get_cloned().is_some() {
                v
            } else {
                None
            }
        }))
    }

    fn transition(&self, direction: OutgoingItemDirection) {
        let current = self.current_item_index.get();

        let next = match direction {
            OutgoingItemDirection::Left => {
                (self.source.total_count() + current - 1) % self.source.total_count()
            }
            OutgoingItemDirection::Right => (current + 1) % self.source.total_count(),
        };

        let outgoing = self.outgoing_item.clone();

        let active_item = (self.current_active_child_element.get() + 1) % 2;
        self.current_active_child_element.set(active_item);
        self.current_item_index.set(next);
        self.outgoing_item.set(Some(OutgoingItem {
            index: current,
            direction,
        }));

        let f = Closure::wrap(Box::new(clone!(outgoing => move || {
            outgoing.set(None);
        })) as Box<dyn Fn()>);

        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&f.as_ref().unchecked_ref(), 500)
            .unwrap();

        f.forget()
    }
}

impl<T: CarouselSource> Carousel<T> {
    pub fn new(source: T) -> Carousel<T> {
        Carousel {
            current_item_index: Mutable::new(0),
            outgoing_item: Mutable::new(Some(OutgoingItem {
                index: 0,
                direction: OutgoingItemDirection::Left,
            })),
            current_active_child_element: Mutable::new(0),
            source: Rc::new(source),
        }
    }

    #[inline]
    pub fn render(mut self) -> Dom {
        self.render_apply(|d, _| d)
    }

    pub fn render_apply<F>(self, cb: F) -> Dom
    where
        F: FnOnce(DomBuilder<HtmlElement>, Rc<Carousel<T>>) -> DomBuilder<HtmlElement>,
    {
        let state = Rc::new(self);

        Dom::with_state(state, |state| {
            html!("div", {
                .apply(clone!(state => move |dom| { cb(dom, state) }))
                .class("dmat-carousel")
                .child(html!("div", {
                    .class("container")
                    .children(&mut [
                        html!("div", {
                            .class_signal("-leave-left", state.child_leave_signal(0, OutgoingItemDirection::Left))
                            .class_signal("-leave-right", state.child_leave_signal(0, OutgoingItemDirection::Right))
                            .class_signal("-hidden", state.hidden_signal(0))
                            .class("dmat-carousel-item")
                            .child(html!("div", {
                                .class("dmat-carousel-item-inner")
                                .child_signal(state.child_signal(0))
                            }))
                        }),
                        html!("div", {
                            .class_signal("-leave-left", state.child_leave_signal(1, OutgoingItemDirection::Left))
                            .class_signal("-leave-right", state.child_leave_signal(1, OutgoingItemDirection::Right))
                            .class_signal("-hidden", state.hidden_signal(1))
                            .class("dmat-carousel-item")
                            .child(html!("div", {
                                .class("dmat-carousel-item-inner")
                                .child_signal(state.child_signal(1))
                            }))
                        }),
                        html!("div", {
                            .class("dmat-carousel-left-button")
                            .class("dmat-carousel-button")
                            .event(clone!(state => {
                                move |_: events::Click| {
                                    state.transition(OutgoingItemDirection::Left);
                                }
                            }))
                        }),
                        html!("div", {
                            .class("dmat-carousel-right-button")
                            .class("dmat-carousel-button")
                            .event(clone!(state => {
                                move |_: events::Click| {
                                    state.transition(OutgoingItemDirection::Right);
                                }
                            }))
                        })
                    ])
                }))
            })
        })
    }
}
