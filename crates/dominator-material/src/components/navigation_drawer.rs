use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, MutableSignalCloned};
use futures_signals::signal_vec::MutableVec;
use futures_signals::signal_vec::SignalVecExt;
use wasm_bindgen::__rt::std::rc::Rc;
use web_sys::HtmlElement;

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

pub type TitleViewGenerator<T> = dyn Fn(&Option<T>, &Rc<NavigationDrawerProps<T>>) -> Option<Dom>;
pub type MainViewGenerator<T> = dyn Fn(&Option<T>, &Rc<NavigationDrawerProps<T>>) -> Option<Dom>;

#[derive(Default)]
pub struct NavigationDrawerProps<T: Clone + PartialEq + 'static> {
    pub entries: MutableVec<NavigationDrawerEntry<T>>,
    pub main_view_generator: Option<Rc<MainViewGenerator<T>>>,
    pub title_view_generator: Option<Rc<TitleViewGenerator<T>>>,
    pub show_toggle_controls: bool,
    pub is_modal: bool,
    pub expanded: Mutable<bool>,
    pub current_active: Mutable<Option<T>>,
}

impl<T: Clone + PartialEq + 'static> NavigationDrawerProps<T> {
    pub fn set_entries(&self, entries: Vec<NavigationDrawerEntry<T>>) {
        self.entries.lock_mut().replace_cloned(entries);
    }

    fn activate_entry(&self, id: T) {
        self.current_active.set(Some(id));

        if self.is_modal {
            self.expanded.set(false);
        }
    }

    fn toggle(&self, state: bool) {
        self.expanded.set(state);
    }
}

impl<T: Clone + PartialEq + 'static> NavigationDrawerProps<T> {
    pub fn new() -> NavigationDrawerProps<T> {
        NavigationDrawerProps {
            entries: Default::default(),
            current_active: Mutable::new(None),
            main_view_generator: None,
            title_view_generator: None,
            show_toggle_controls: false,
            is_modal: false,
            expanded: Mutable::new(true),
        }
    }

    #[inline]
    #[must_use]
    pub fn main_view_generator<S>(mut self, main_view_generator: S) -> Self
    where
        S: Fn(&Option<T>, &Rc<NavigationDrawerProps<T>>) -> Option<Dom> + 'static,
    {
        self.main_view_generator = Some(Rc::new(main_view_generator));
        self
    }

    #[inline]
    #[must_use]
    pub fn title_view_generator<S>(mut self, title_view_generator: S) -> Self
    where
        S: Fn(&Option<T>, &Rc<NavigationDrawerProps<T>>) -> Option<Dom> + 'static,
    {
        self.title_view_generator = Some(Rc::new(title_view_generator));
        self
    }

    #[inline]
    #[must_use]
    pub fn expanded(self, expanded: bool) -> Self {
        self.expanded.set(expanded);
        self
    }

    #[inline]
    #[must_use]
    pub fn show_toggle_controls(mut self, show_toggle_controls: bool) -> Self {
        self.show_toggle_controls = show_toggle_controls;
        self
    }

    #[inline]
    #[must_use]
    pub fn modal(mut self, is_modal: bool) -> Self {
        self.is_modal = is_modal;
        self
    }

    #[inline]
    #[must_use]
    pub fn initial_selected(self, initial: T) -> Self {
        self.current_active.set(Some(initial));
        self
    }

    #[inline]
    #[must_use]
    pub fn entries(self, entries: Vec<NavigationDrawerEntry<T>>) -> Self {
        self.entries.lock_mut().replace_cloned(entries);
        self
    }
}

pub struct NavigationDrawerOut {
    pub is_expanded: MutableSignalCloned<bool>,
}

#[macro_export]
macro_rules! navigation_drawer {
    ($props: expr) => {{
        $crate::components::navigation_drawer::navigation_drawer($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::navigation_drawer::navigation_drawer($props, $mixin)
    }};
}

pub fn navigation_drawer<T: Clone + PartialEq + 'static, F>(
    props: NavigationDrawerProps<T>,
    mixin: F,
) -> (Dom, Rc<NavigationDrawerOut>)
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let out = Rc::new(NavigationDrawerOut {
        is_expanded: props.expanded.signal_cloned(),
    });

    let s = Rc::new(props);

    (
        html!("div", {
            .class("dmat-navigation-drawer-container")
            .apply(mixin)
            .children(vec![
                match s.main_view_generator.clone() {
                    Some(generator) => {
                        let exp = s.expanded.signal_cloned();
                        let active = s.current_active.signal_cloned();
                        let state = s.clone();

                        Some(html!("div", {
                            .class("main")
                            .class_signal("-expanded", s.expanded.signal())
                            .apply_if(s.is_modal, |dom| dom.class("-modal"))
                            .class("dmat-surface")
                            .child_signal(map_ref!{ let active = active, let expanded = exp => move {
                                Some(html!("div", {
                                    .children(vec![
                                        generator(active, &state),
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
                                        }
                                    ].into_iter().flatten())
                                }))
                            }})
                        }))
                    },
                    _ => None
                },

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
                }))].into_iter().flatten())
        }),
        out,
    )
}
