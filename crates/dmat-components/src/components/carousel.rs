use dominator::{clone, events, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal};
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::futures_signals::signal::SignalExt;

#[component(render_fn = carousel)]
pub struct Carousel<TItemRenderFn: Fn(i32) -> Dom = fn(i32) -> Dom> {
    pub item_render_fn: TItemRenderFn,
}

#[inline]
pub fn carousel(props: impl CarouselPropsTrait + 'static) -> Dom {
    let CarouselProps {
        item_render_fn,
        apply,
    } = props.take();

    let state = Rc::new(Carousel {
        current_item_index: Mutable::new(0),
        previous_item_index: Mutable::new(0),
        is_transitioning: Mutable::new(false),
    });

    let item_render_fn = Rc::new(item_render_fn.expect_throw("item_render_fn missing"));

    let child_odd_signal =
        state
            .current_item_index
            .signal()
            .filter_map(clone!(item_render_fn => move |v| {
                if v.abs() % 2 == 1 {
                    Some((*item_render_fn)(v))
                } else {
                    None
                }
            }));

    let child_even_signal =
        state
            .current_item_index
            .signal()
            .filter_map(clone!(item_render_fn => move |v| {
                if v.abs() % 2 == 0 {
                    Some((*item_render_fn)(v))
                } else {
                    None
                }
            }));

    html!("div", {
        .class("dmat-carousel")
        .child(html!("div", {
            .class("container")
            .apply_if(apply.is_some(), |dom| dom.apply(apply.unwrap_throw()))
            .children(&mut [
                carousel_item(
                    child_even_signal,
                    state.hidden_signal(0),
                    state.child_leave_signal(0, OutgoingItemDirection::Left),
                    state.child_leave_signal(0, OutgoingItemDirection::Right)
                ),
                carousel_item(
                    child_odd_signal,
                    state.hidden_signal(1),
                    state.child_leave_signal(1, OutgoingItemDirection::Left),
                    state.child_leave_signal(1, OutgoingItemDirection::Right)
                ),
                carousel_button(clone!(state => {
                    move |_: events::Click| {
                        state.transition(OutgoingItemDirection::Left)
                    }
                }), "left"),
                carousel_button(clone!(state => {
                    move |_: events::Click| {
                        state.transition(OutgoingItemDirection::Right)
                    }
                }), "right")
            ])
        }))
    })
}

#[derive(Clone, Eq, PartialEq)]
pub enum OutgoingItemDirection {
    Left,
    Right,
}

#[derive(Default)]
struct Carousel {
    pub current_item_index: Mutable<i32>,
    pub previous_item_index: Mutable<i32>,
    pub is_transitioning: Mutable<bool>,
}

impl Carousel {
    /// This produces a signal which will yield true if the child at `child_index`
    /// is considered leaving in `direction`
    fn child_leave_signal(
        &self,
        oddity: i32,
        direction: OutgoingItemDirection,
    ) -> impl Signal<Item = bool> {
        map_ref!(
            let current = self.current_item_index.signal(),
            let previous = self.previous_item_index.signal() => move {
                if current.abs() % 2 == oddity {
                    false
                } else if *current < *previous {
                    direction == OutgoingItemDirection::Right
                } else {
                    direction == OutgoingItemDirection::Left
                }
            }
        )
    }

    fn hidden_signal(&self, oddity: i32) -> impl Signal<Item = bool> {
        map_ref!(
            let transitioning = self.is_transitioning.signal(),
            let current = self.current_item_index.signal() => move {
                (!transitioning) && current.abs() % 2 != oddity
            }
        )
    }

    fn transition(&self, direction: OutgoingItemDirection) {
        self.previous_item_index.set(self.current_item_index.get());

        match direction {
            OutgoingItemDirection::Left => {
                self.current_item_index.replace_with(|v| *v - 1);
            }
            OutgoingItemDirection::Right => {
                self.current_item_index.replace_with(|v| *v + 1);
            }
        }

        let is_transitioning = self.is_transitioning.clone();
        self.is_transitioning.set(true);

        let transition_end_cb = Closure::wrap(Box::new(move || {
            is_transitioning.set(false);
        }) as Box<dyn Fn()>);

        web_sys::window().map(|window| {
            window.set_timeout_with_callback_and_timeout_and_arguments_0(
                transition_end_cb.as_ref().unchecked_ref(),
                200,
            )
        });

        transition_end_cb.forget()
    }
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
