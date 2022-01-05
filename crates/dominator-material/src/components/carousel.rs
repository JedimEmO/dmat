use std::error::Error;

use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, MutableSignal, Signal};
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::futures_signals::signal::SignalExt;

pub trait CarouselSource: Clone {
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

struct Carousel<T: CarouselSource + 'static> {
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

    fn transition(&self, direction: OutgoingItemDirection, target_idx: Option<usize>) {
        let current = self.current_item_index.get();

        let next = match target_idx {
            Some(next_idx) => next_idx,
            _ => match direction {
                OutgoingItemDirection::Left => {
                    (self.source.total_count() + current - 1) % self.source.total_count()
                }
                OutgoingItemDirection::Right => (current + 1) % self.source.total_count(),
            },
        };

        let outgoing = self.outgoing_item.clone();

        let active_item = (self.current_active_child_element.get() + 1) % 2;
        self.current_active_child_element.set(active_item);
        self.current_item_index.set(next);
        self.outgoing_item.set(Some(OutgoingItem {
            index: current,
            direction,
        }));

        let transition_end_cb = Closure::wrap(Box::new(clone!(outgoing => move || {
            outgoing.set(None);
        })) as Box<dyn Fn()>);

        web_sys::window().map(|window| {
            window.set_timeout_with_callback_and_timeout_and_arguments_0(
                transition_end_cb.as_ref().unchecked_ref(),
                200,
            )
        });

        transition_end_cb.forget()
    }
}

#[derive(Clone)]
pub struct CarouselControls<T: CarouselSource + 'static> {
    carousel: Rc<Carousel<T>>,
}

impl<T: CarouselSource + 'static> CarouselControls<T> {
    fn new(carousel: Rc<Carousel<T>>) -> CarouselControls<T> {
        CarouselControls { carousel }
    }

    pub fn goto_index(&self, idx: usize) -> Result<(), Box<dyn Error>> {
        self.carousel
            .transition(OutgoingItemDirection::Right, Some(idx));
        Ok(())
    }
}

pub struct CarouselProps<T: CarouselSource> {
    pub source: T,
    pub initial_view_index: usize,
}

pub fn carousel<
    T: CarouselSource + 'static,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
>(
    props: CarouselProps<T>,
    mixin: F,
) -> (Dom, CarouselControls<T>) {
    let source = props.source;

    let state = Rc::new(Carousel {
        current_item_index: Mutable::new(props.initial_view_index),
        outgoing_item: Mutable::new(Some(OutgoingItem {
            index: 0,
            direction: OutgoingItemDirection::Left,
        })),
        current_active_child_element: Mutable::new(0),

        source: Rc::new(source),
    });

    (
        html!("div", {.class("dmat-carousel").child(html!("div", {
            .class("container")
            .apply(mixin)
            .children(&mut [
                carousel_item(
                    state.child_signal(0),
                    state.hidden_signal(0),
                    state.child_leave_signal(0, OutgoingItemDirection::Left),
                    state.child_leave_signal(0, OutgoingItemDirection::Right)
                ),
                carousel_item(
                    state.child_signal(1),
                    state.hidden_signal(1),
                    state.child_leave_signal(1, OutgoingItemDirection::Left),
                    state.child_leave_signal(1, OutgoingItemDirection::Right)
                ),
                carousel_button(clone!(state => {
                        move |_: events::Click| {
                            state.transition(OutgoingItemDirection::Left, None);
                        }
                    }), "left"),
                carousel_button(clone!(state => {
                        move |_: events::Click| {
                            state.transition(OutgoingItemDirection::Right, None);
                        }
                    }), "right")
            ])
        }))
        }),
        CarouselControls::new(state),
    )
}

#[inline]
fn carousel_button<F: Fn(events::Click) + 'static>(f: F, dir: &str) -> Dom {
    html!("div", {
        .class(format!("dmat-carousel-{}-button", dir).as_str())
        .class("dmat-carousel-button")
        .event(f)
    })
}

#[inline]
fn carousel_item<
    TChild: Signal<Item = Option<Dom>> + 'static,
    THidden: Signal<Item = bool> + 'static,
    TLeaveLeft: Signal<Item = bool> + 'static,
    TLeaveRight: Signal<Item = bool> + 'static,
>(
    child: TChild,
    hidden: THidden,
    leave_left: TLeaveLeft,
    leave_right: TLeaveRight,
) -> Dom {
    html!("div", {
        .class_signal("-leave-left", leave_left)
        .class_signal("-leave-right", leave_right)
        .class_signal("-hidden", hidden)
        .class("dmat-carousel-item")
        .child(html!("div", {
            .class("dmat-carousel-item-inner")
            .child_signal(child)
        }))
    })
}
