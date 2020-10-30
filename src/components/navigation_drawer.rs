use std::borrow::BorrowMut;
use std::ops::DerefMut;

use dominator::{clone, Dom, events, html};
use futures_signals::signal::{Mutable, Signal};
use futures_signals::signal_vec::MutableVec;
use futures_signals::signal_vec::SignalVecExt;
use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::core::pin::Pin;
use wasm_bindgen::__rt::std::rc::Rc;

#[derive(Clone)]
pub struct NavigationEntry<T: Clone + 'static> {
    pub id: T,
    pub text: String,
}

#[derive(Clone)]
pub enum NavigationDrawerEntry<T: Clone + 'static> {
    Item(NavigationEntry<T>),
    Separator,
}

pub struct NavigationDrawerData<T: Clone + PartialEq + 'static> {
    entries: MutableVec<NavigationDrawerEntry<T>>,
    main_view_generator: Option<Rc<dyn Fn(&Option<T>, &Rc<NavigationDrawerData<T>>) -> Option<Dom>>>,
    title_view_generator: Option<Rc<dyn Fn(&Option<T>, &Rc<NavigationDrawerData<T>>) -> Option<Dom>>>,
    pub expanded: Mutable<bool>,
    pub current_active: Mutable<Option<T>>,
}

impl<T: Clone + PartialEq + 'static> NavigationDrawerData<T> {
    pub fn set_entries(&self, entries: Vec<NavigationDrawerEntry<T>>) {
        self.entries.lock_mut().replace_cloned(entries);
    }
}

pub struct NavigationDrawer<T: Clone + PartialEq + 'static> {
    state_handler: Option<Box<dyn FnOnce(Rc<NavigationDrawerData<T>>)>>,
    data: NavigationDrawerData<T>,
}

impl<T: Clone + PartialEq + 'static> NavigationDrawer<T> {
    pub fn new() -> NavigationDrawer<T> {
        NavigationDrawer {
            state_handler: None,
            data: NavigationDrawerData {
                entries: Default::default(),
                current_active: Mutable::new(None),
                main_view_generator: None,
                title_view_generator: None,
                expanded: Mutable::new(true),
            },
        }
    }

    #[inline]
    pub fn main_view_generator<S>(mut self, main_view_generator: S) -> Self where S: Fn(&Option<T>, &Rc<NavigationDrawerData<T>>) -> Option<Dom> + 'static {
        self.data.main_view_generator = Some(Rc::new(main_view_generator));
        self
    }

    #[inline]
    pub fn title_view_generator<S>(mut self, title_view_generator: S) -> Self where S: Fn(&Option<T>, &Rc<NavigationDrawerData<T>>) -> Option<Dom> + 'static {
        self.data.title_view_generator = Some(Rc::new(title_view_generator));
        self
    }

    #[inline]
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.data.expanded.set(expanded);
        self
    }

    #[inline]
    pub fn initial_active(mut self, initial: T) -> Self {
        self.data.current_active.set(Some(initial));
        self
    }

    pub fn render(mut self) -> Dom {
        self.render_with_handle().1
    }

    pub fn render_with_handle(mut self) -> (Rc<NavigationDrawerData<T>>, Dom) {
        let s = Rc::new(self.data);

        if let Some(state_handler) = self.state_handler {
            state_handler(s.clone())
        }

        (s.clone(), Dom::with_state(s, |s| {
            html!("div", {
                .class("dmat-navigation-drawer-container")
                .children(vec![
                    Some(html!("div", {
                        .class("drawer")
                        .class_signal("-expanded", s.expanded.signal())
                        .child(html!("div", {
                            .class("drawer-container")
                            .children(&mut [
                                match &s.title_view_generator {
                                    Some(generator) => html!("div", {
                                        .class("title")
                                        .child_signal(s.current_active.signal_ref(clone!(generator, s => move |v| generator(v, &s))))
                                    }),
                                    _ => html!("span")
                                },
                                html!("div", {
                                    .children_signal_vec(clone!(s => s.entries.signal_vec_cloned().map(move |entry| {
                                        match entry {
                                            NavigationDrawerEntry::Item(v) => {
                                                html!("div", {
                                                    .class("entry")
                                                    .class_signal("-active", s.current_active.signal_ref(clone!(v => move |active|{
                                                        match active {
                                                            Some(b) => *b == v.id.clone(),
                                                            _ => false
                                                        }
                                                    })))
                                                    .text(v.text.as_str())
                                                    .event(clone!(s => move |_: events::Click| {
                                                        s.current_active.set(Some(v.id.clone()))
                                                    }))
                                                })
                                            },
                                            _ => html!("div", { .class("dmat-separator") })
                                        }
                                    })))
                                })
                            ])
                        }))
                    })),
                    match &s.main_view_generator {
                        Some(generator) => Some(html!("div", {
                            .class("main")
                            .class_signal("-expanded", s.expanded.signal())
                            .class("dmat-surface")
                            .child_signal(s.current_active.signal_ref(clone!(generator, s => move |v| generator(v, &s))))
                        })),
                        _ => None
                    }
                ].into_iter().filter_map(|v| v))
            })
        })
        )
    }
}
