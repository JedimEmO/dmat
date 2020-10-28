use std::borrow::BorrowMut;
use std::ops::DerefMut;

use dominator::{Dom, html};
use futures_signals::signal::{Mutable, Signal};
use futures_signals::signal_vec::MutableVec;
use futures_signals::signal_vec::SignalVecExt;
use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::core::pin::Pin;
use wasm_bindgen::__rt::std::rc::Rc;

#[derive(Clone)]
pub enum NavigationDrawerEntry {
    Item
}

pub struct NavigationDrawerData {
    main_signal: Option<Pin<Box<dyn Signal<Item=Option<Dom>>>>>,
    entries: MutableVec<NavigationDrawerEntry>,
}

impl NavigationDrawerData {
    pub fn set_entries(&self, entries: Vec<NavigationDrawerEntry>) {
        self.entries.lock_mut().replace_cloned(entries);
    }
}

pub struct NavigationDrawer {
    state_handler: Option<Box<dyn FnOnce(Rc<NavigationDrawerData>)>>,
    data: NavigationDrawerData,
}

impl NavigationDrawer {
    pub fn new() -> NavigationDrawer {
        NavigationDrawer {
            state_handler: None,
            data: NavigationDrawerData {
                main_signal: None,
                entries: Default::default(),
            },
        }
    }

    #[inline]
    pub fn main<S>(mut self, main_signal: S) -> Self where S: Signal<Item=Option<Dom>> + 'static {
        self.data.main_signal = Some(Box::pin(main_signal));
        self
    }

    #[inline]
    pub fn connect<F: 'static>(mut self, state_handler: F) -> Self where F: FnOnce(Rc<NavigationDrawerData>) {
        self.state_handler = Some(Box::new(state_handler));
        self
    }

    pub fn render(mut self) -> Dom {
        let s = Rc::new(self.data);

        if let Some(state_handler) = self.state_handler {
            state_handler(s.clone())
        }

        Dom::with_state(s, |s| {
            html!("div", {
                .class("dmat-navigation-drawer-container")
                .children(vec![
                    Some(html!("div", {
                        .class("dmat-navigation-drawer")
                        .child(html!("div", {
                            .class("drawer-container")
                            .children_signal_vec(s.entries.signal_vec_cloned().map(|entry| {
                                html!("div", { .class("dmat-navigation-drawer-entry") .text("Some entry") })
                            }))
                        }))
                    })),
                    match Rc::get_mut(s).unwrap().main_signal.take() {
                        Some(sig) => Some(html!("div", {
                            .class("dmat-navigation-drawer-main")
                            .class("dmat-surface")
                            .child_signal(sig)
                        })),
                        _ => None
                    }
                ].into_iter().filter_map(|v| v))
            })
        })
    }
}
