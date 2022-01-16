use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, MutableSignalCloned, ReadOnlyMutable, Signal};
use futures_signals::signal_vec::SignalVecExt;
use futures_signals::signal_vec::{MutableSignalVec, MutableVec};
use wasm_bindgen::__rt::std::rc::Rc;
use web_sys::HtmlElement;

use crate::futures_signals::signal::SignalExt;

#[derive(Clone)]
pub enum NavigationDrawerEntry<T: Clone + Copy + 'static> {
    Item(T),
    Separator,
}

#[derive(Copy, Clone, PartialEq)]
pub enum DrawerWidth {
    Full,
    Narrow,
}

pub struct NavigationDrawerProps<
    T: Clone + Copy + PartialEq + 'static,
    FMain: Fn(&Option<T>) -> Option<Dom>,
    FItem: Fn(T, DrawerWidth) -> Dom,
> {
    pub entries: MutableVec<NavigationDrawerEntry<T>>,
    pub main_view_generator: FMain,
    pub header_view_generator: Option<Rc<dyn Fn(Option<T>, Mutable<bool>) -> Option<Dom>>>,
    pub item_renderer: FItem,
    pub show_toggle_controls: bool,
    pub is_modal: bool,
    pub expanded: Mutable<bool>,
    pub current_active: Mutable<Option<T>>,
    pub width: ReadOnlyMutable<DrawerWidth>,
}

impl<
        T: Clone + Copy + PartialEq + 'static,
        FMain: Fn(&Option<T>) -> Option<Dom>,
        FItem: Fn(T, DrawerWidth) -> Dom,
    > NavigationDrawerProps<T, FMain, FItem>
{
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

impl<
        T: Clone + Copy + PartialEq + 'static,
        FMain: Fn(&Option<T>) -> Option<Dom>,
        FItem: Fn(T, DrawerWidth) -> Dom,
    > NavigationDrawerProps<T, FMain, FItem>
{
    pub fn new(
        main_view_generator: FMain,
        item_renderer: FItem,
    ) -> NavigationDrawerProps<T, FMain, FItem> {
        NavigationDrawerProps {
            entries: Default::default(),
            current_active: Mutable::new(None),
            main_view_generator,
            header_view_generator: None,
            item_renderer,
            show_toggle_controls: false,
            is_modal: false,
            expanded: Mutable::new(true),
            width: Mutable::new(DrawerWidth::Full).read_only(),
        }
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

    #[inline]
    #[must_use]
    pub fn header_view_generator<F>(mut self, header_view_generator: F) -> Self
    where
        F: Fn(Option<T>, Mutable<bool>) -> Option<Dom> + 'static,
    {
        self.header_view_generator = Some(Rc::new(header_view_generator));
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

pub fn navigation_drawer<
    T: Clone + Copy + PartialEq + 'static,
    F,
    FMain: Fn(&Option<T>) -> Option<Dom> + 'static,
    FItem: Fn(T, DrawerWidth) -> Dom + 'static,
>(
    props: NavigationDrawerProps<T, FMain, FItem>,
    mixin: F,
) -> (Dom, NavigationDrawerOut)
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let out = NavigationDrawerOut {
        is_expanded: props.expanded.signal_cloned(),
    };

    let current_active = props.current_active;
    let expanded = props.expanded;
    let header_view_generator = props.header_view_generator;
    let configured_width = props.width;

    let current_width = Mutable::new(configured_width.get());

    (
        html!("div", {
            .class("dmat-navigation-drawer-container")
            .apply(mixin)
            .class_signal("-expanded", expanded.signal())
            .class_signal("-narrow", current_width.signal().map(|w| match w {
                DrawerWidth::Narrow => true,
                _ => false
            }))
            .children(vec![
                Some(html!("div", {
                    .class("drawer")
                    .class_signal("-expanded", expanded.signal())
                    .event(clone!(current_width => move |_:events::MouseEnter| {
                            current_width.set(DrawerWidth::Full)
                        }))
                    .event(clone!(current_width, configured_width=> move |_:events::MouseLeave| {
                        if configured_width.get() == DrawerWidth::Narrow {
                            current_width.set(DrawerWidth::Narrow)
                        }
                    }))

                    .child(html!("div", {
                        .class("drawer-container")
                        .children(&mut [
                            controls(expanded.clone(), props.show_toggle_controls),
                            header(header_view_generator, current_active.clone(), expanded.clone()),
                            items(props.item_renderer, props.entries.signal_vec_cloned(), current_active.clone(), current_width.read_only())
                        ])
                    }))
                })),
                main_view(
                    props.main_view_generator,
                    expanded.clone(),
                    current_active.signal_cloned(),
                    props.is_modal,
                    props.show_toggle_controls
                )
            ].into_iter().flatten())
        }),
        out,
    )
}

#[inline]
fn header<T: Clone + PartialEq + 'static>(
    header_view_generator: Option<Rc<dyn Fn(Option<T>, Mutable<bool>) -> Option<Dom>>>,
    current_active: Mutable<Option<T>>,
    expanded: Mutable<bool>,
) -> Dom {
    match header_view_generator {
        Some(generator) => html!("div", {
            .class("title")
            .child_signal(current_active.signal_cloned().map(clone!(generator, expanded => move |v| generator(v, expanded.clone()))))
        }),
        _ => html!("span"),
    }
}

#[inline]
fn controls(expanded: Mutable<bool>, show_toggle_controls: bool) -> Dom {
    html!("div", {
        .class("controls")
        .child_signal(expanded.signal_cloned().map(clone!(expanded => move |is_expanded| {
            if is_expanded && show_toggle_controls {
                Some(html!("div", {
                    .child(html!("span", {
                        .class("dmat-navigation-drawer-collapse")
                        .event(clone!(expanded => move |_:events::Click| {
                            expanded.set(false);
                        }))
                    }))
                }))
            } else {
                None
            }
        })))
    })
}

#[inline]
fn items<T: Clone + Copy + PartialEq + 'static, FItem: Fn(T, DrawerWidth) -> Dom + 'static>(
    render_item: FItem,
    entries: MutableSignalVec<NavigationDrawerEntry<T>>,
    active: Mutable<Option<T>>,
    width: ReadOnlyMutable<DrawerWidth>,
) -> Dom {
    let render_item = Rc::new(render_item);

    html!("div", {
        .children_signal_vec(entries.map(clone!(width => move |entry| {
            match entry {
                NavigationDrawerEntry::Item(v) => {
                    html!("div", {
                        .class("entry")
                        .class_signal("-active", active.signal_cloned().map(clone!(v => move |active|{
                            match active {
                                Some(b) => b == v,
                                _ => false
                            }
                        })))
                        .child_signal(width.signal_cloned().map({
                            clone!(v, render_item => move |w| {
                                let render_item = &render_item;
                                Some(render_item(v, w))
                            })
                        }))
                        .event(clone!(active, v => move |_: events::Click| {
                            active.set(Some(v.clone()))
                        }))
                    })
                },
                _ => html!("div", { .class("dmat-separator") })
            }
        })))
    })
}

#[inline]
fn main_view<T, F, TActive>(
    renderer: F,
    expanded: Mutable<bool>,
    active: TActive,
    is_modal: bool,
    show_toggle_controls: bool,
) -> Option<Dom>
where
    T: Clone + PartialEq + 'static,
    F: Fn(&Option<T>) -> Option<Dom> + 'static,
    TActive: Signal<Item = Option<T>> + 'static,
{
    Some(html!("div", {
        .class("main")
        .class_signal("-expanded", expanded.signal_cloned())
        .apply_if(is_modal, |dom| dom.class("-modal"))
        .class("dmat-surface")
        .child_signal(map_ref!{ let active = active, let is_expanded = expanded.signal_cloned() => move {
            Some(html!("div", {
                .children(vec![
                    renderer(active),
                    match !is_expanded && show_toggle_controls {
                        true => Some(html!("span", {
                                .class("dmat-navigation-drawer-expand")
                                .event(clone!(expanded => move |_:events::Click| {
                                    expanded.set(true);
                                }))
                            }))                                                ,
                        false => None
                    },
                    match is_modal && *is_expanded {
                        true => Some(html!("div", {
                            .class("dmat-modal-cover")
                            .event(clone!(expanded => move |_: events::Click| {
                                expanded.set(false);
                            }))
                        })),
                        false => None
                    }
                ].into_iter().flatten())
            }))
        }})
    }))
}
