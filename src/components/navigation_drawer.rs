use std::borrow::BorrowMut;
use std::ops::DerefMut;

use dominator::{clone, events, html, Dom, svg};
use futures_signals::map_ref;
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
    main_view_generator:
        Option<Rc<dyn Fn(&Option<T>, &Rc<NavigationDrawerData<T>>) -> Option<Dom>>>,
    title_view_generator:
        Option<Rc<dyn Fn(&Option<T>, &Rc<NavigationDrawerData<T>>) -> Option<Dom>>>,
    show_toggle_controls: bool,
    is_modal: bool,
    pub expanded: Mutable<bool>,
    pub current_active: Mutable<Option<T>>,
}

impl<T: Clone + PartialEq + 'static> NavigationDrawerData<T> {
    pub fn set_entries(&self, entries: Vec<NavigationDrawerEntry<T>>) {
        self.entries.lock_mut().replace_cloned(entries);
    }

    fn activate_entry(&self, id: T) {
        self.current_active.set(Some(id.clone()));

        if self.is_modal {
            self.expanded.set(false);
        }
    }

    fn toggle(&self, state: bool) {
        self.expanded.set(state);
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
                show_toggle_controls: false,
                is_modal: false,
                expanded: Mutable::new(true),
            },
        }
    }

    #[inline]
    pub fn main_view_generator<S>(mut self, main_view_generator: S) -> Self
    where
        S: Fn(&Option<T>, &Rc<NavigationDrawerData<T>>) -> Option<Dom> + 'static,
    {
        self.data.main_view_generator = Some(Rc::new(main_view_generator));
        self
    }

    #[inline]
    pub fn title_view_generator<S>(mut self, title_view_generator: S) -> Self
    where
        S: Fn(&Option<T>, &Rc<NavigationDrawerData<T>>) -> Option<Dom> + 'static,
    {
        self.data.title_view_generator = Some(Rc::new(title_view_generator));
        self
    }

    #[inline]
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.data.expanded.set(expanded);
        self
    }

    #[inline]
    pub fn show_toggle_controls(mut self, show_toggle_controls: bool) -> Self {
        self.data.show_toggle_controls = show_toggle_controls;
        self
    }

    #[inline]
    pub fn modal(mut self, is_modal: bool) -> Self {
        self.data.is_modal = is_modal;
        self
    }

    #[inline]
    pub fn initial_selected(mut self, initial: T) -> Self {
        self.data.current_active.set(Some(initial));
        self
    }

    #[inline]
    pub fn entries(mut self, entries: Vec<NavigationDrawerEntry<T>>) -> Self {
        self.data.entries.lock_mut().replace_cloned(entries);
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

        (
            s.clone(),
            Dom::with_state(s, |s| {
                html!("div", {
                    .class("dmat-navigation-drawer-container")
                    .children(vec![
                        Some(html!("div", {
                            .class("drawer")
                            .class_signal("-expanded", s.expanded.signal())
                            .child(html!("div", {
                                .class("drawer-container")
                                .children(&mut [
                                    match s.expanded.get() && s.show_toggle_controls {
                                        true => html!("div", {
                                            .class("controls")
                                            .child(html!("span", {
                                                .class("dmat-navigation-drawer-collapse")
                                                .event(clone!(s => move |_:events::Click| {
                                                    s.toggle(false);
                                                }))
                                            }))
                                        }),
                                        false => html!("span")
                                    },
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
                                                            s.activate_entry(v.id.clone())
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
                        match s.main_view_generator.clone() {
                            Some(generator) => {
                                let exp = s.expanded.signal_cloned();
                                let active = s.current_active.signal_cloned();
                                let state = s.clone();

                                Some(html!("div", {
                                    .class("main")
                                    .class_signal("-expanded", s.expanded.signal())
                                    .class("dmat-surface")
                                    .child_signal(map_ref!{ let active = active, let expanded = exp => move {
                                        Some(html!("div", {
                                            .children(vec![
                                                match !*expanded && state.show_toggle_controls {
                                                    true => Some(html!("span", {
                                                            .class("dmat-navigation-drawer-expand")
                                                            .event(clone!(state => move |_:events::Click| {
                                                                state.toggle(true);
                                                            }))
                                                        }))                                                ,
                                                    false => None
                                                },
                                                match state.is_modal && *expanded {
                                                    true => Some(html!("div", {
                                                        .class("dmat-modal-cover")
                                                        .event(clone!(state => move |_: events::Click| {
                                                            state.expanded.set(false);
                                                        }))
                                                    })),
                                                    false => None
                                                },
                                                generator(active, &state)
                                            ].into_iter().filter_map(|v| v))
                                        }))
                                    }})
                                }))
                            },
                            _ => None
                        }
                    ].into_iter().filter_map(|v| v))
                })
            }),
        )
    }
}
