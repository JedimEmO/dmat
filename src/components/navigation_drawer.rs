use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::RwLock;
use std::task::{Context, Poll};

use crate::elements::elements::new_html;
use dominator::{clone, events, html, Dom, DomBuilder};

use futures_signals::signal::{Mutable, MutableSignalCloned, Signal};
use futures_signals::signal_vec::MutableVec;
use futures_signals::signal_vec::SignalVecExt;
use futures_signals::{map_ref, unsafe_project};
use wasm_bindgen::__rt::std::rc::Rc;
use web_sys::{Element, HtmlElement};

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

pub struct NavigationDrawerProps<T: Clone + PartialEq + 'static> {
    pub entries: MutableVec<NavigationDrawerEntry<T>>,
    pub main_view_generator:
        Option<Rc<dyn Fn(&Option<T>, &Rc<NavigationDrawerProps<T>>) -> Option<Dom>>>,
    pub title_view_generator:
        Option<Rc<dyn Fn(&Option<T>, &Rc<NavigationDrawerProps<T>>) -> Option<Dom>>>,
    pub show_toggle_controls: bool,
    pub is_modal: bool,
    pub expanded: Mutable<bool>,
    pub current_active: Mutable<Option<T>>,
    pub apply_func: Option<
        Box<
            dyn FnOnce(
                Rc<NavigationDrawerProps<T>>,
                DomBuilder<HtmlElement>,
            ) -> DomBuilder<HtmlElement>,
        >,
    >,
}

impl<T: Clone + PartialEq + 'static> NavigationDrawerProps<T> {
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
            apply_func: None,
        }
    }

    #[inline]
    pub fn apply<F>(mut self, apply_func: F) -> Self
    where
        F: FnOnce(Rc<NavigationDrawerProps<T>>, DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>
            + 'static,
    {
        self.apply_func = Some(Box::new(apply_func));
        self
    }

    #[inline]
    pub fn main_view_generator<S>(mut self, main_view_generator: S) -> Self
    where
        S: Fn(&Option<T>, &Rc<NavigationDrawerProps<T>>) -> Option<Dom> + 'static,
    {
        self.main_view_generator = Some(Rc::new(main_view_generator));
        self
    }

    #[inline]
    pub fn title_view_generator<S>(mut self, title_view_generator: S) -> Self
    where
        S: Fn(&Option<T>, &Rc<NavigationDrawerProps<T>>) -> Option<Dom> + 'static,
    {
        self.title_view_generator = Some(Rc::new(title_view_generator));
        self
    }

    #[inline]
    pub fn expanded(self, expanded: bool) -> Self {
        self.expanded.set(expanded);
        self
    }

    #[inline]
    pub fn show_toggle_controls(mut self, show_toggle_controls: bool) -> Self {
        self.show_toggle_controls = show_toggle_controls;
        self
    }

    #[inline]
    pub fn modal(mut self, is_modal: bool) -> Self {
        self.is_modal = is_modal;
        self
    }

    #[inline]
    pub fn initial_selected(self, initial: T) -> Self {
        self.current_active.set(Some(initial));
        self
    }

    #[inline]
    pub fn entries(self, entries: Vec<NavigationDrawerEntry<T>>) -> Self {
        self.entries.lock_mut().replace_cloned(entries);
        self
    }
}

pub struct NavigationDrawerOut {
    pub is_expanded: MutableSignalCloned<bool>,
}

pub fn navigation_drawer<T: Clone + PartialEq + 'static>(
    mut props: NavigationDrawerProps<T>,
) -> (Rc<NavigationDrawerOut>, Dom) {
    let apply_func = props.apply_func.take().or(None);

    let out = Rc::new(NavigationDrawerOut {
        is_expanded: props.expanded.signal_cloned(),
    });

    (
        out,
        Dom::with_state(Rc::new(props), move |s| {
            html!("div", {
                .class("dmat-navigation-drawer-container")
                .apply_if(apply_func.is_some(), clone!(s => move |dom| {
                    apply_func.unwrap()(s, dom)
                }))
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
                                        ].into_iter().filter_map(|v| v))
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
                    }))].into_iter().filter_map(|v| v))
            })
        }),
    )
}

struct UiMonad<
    T: Signal<Item = DomBuilder<A>>,
    A: AsRef<Element>,
    C: Fn(DomBuilder<A>) -> DomBuilder<A>,
> {
    signal: T,
    closure: C,
}

impl<T: Signal<Item = DomBuilder<A>>, A: AsRef<Element>, C: Fn(DomBuilder<A>) -> DomBuilder<A>>
    Signal for UiMonad<T, A, C>
{
    type Item = DomBuilder<A>;

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        unsafe_project!(self => {
            pin signal,
            mut closure,
        });

        signal
            .poll_change(cx)
            .map(|opt| opt.map(|value| closure(value)))
    }
}

struct Bind<Tm: Signal<Item = A>, To: Signal<Item = B> + Unpin, A, B, F: Fn(A) -> To> {
    input: Tm,
    current_signal: RwLock<Option<To>>,
    function: F,
    not_used: PhantomData<A>,
}

impl<Tm: Signal<Item = A>, To: Signal<Item = B> + Unpin, A, B, F: Fn(A) -> To> Signal
    for Bind<Tm, To, A, B, F>
{
    type Item = B;

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        unsafe_project!(self => {
            pin input,
            pin current_signal,
            mut function,
        });

        if let Poll::Ready(v) = input.poll_change(cx) {
            if let Some(new_input) = v {
                current_signal
                    .write()
                    .unwrap()
                    .replace((function)(new_input));
            } else {
                current_signal.write().unwrap().take();
            }

            cx.waker().wake_by_ref();
            return Poll::Pending;
        }

        {
            let mut current_signal = current_signal.write().unwrap();

            if current_signal.is_some() {
                return Pin::new(current_signal.as_mut().unwrap()).poll_change(cx);
            }
        }

        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

pub fn bind<Ti: Signal<Item = A>, To: Signal<Item = B> + Unpin, A, F, B>(
    m: Ti,
    f: F,
) -> impl Signal<Item = B>
where
    F: Fn(A) -> To,
{
    Bind {
        input: m,
        current_signal: RwLock::new(None),
        function: f,
        not_used: PhantomData::<A>::default(),
    }
}

#[inline]
fn with_id<B: Signal<Item = DomBuilder<A>>, A: AsRef<Element>>(
    b: B,
    id: &str,
) -> impl Signal<Item = DomBuilder<A>> {
    let i = id.to_string();

    UiMonad {
        signal: b,
        closure: move |d| d.attribute("id", i.as_str()),
    }
}

pub fn test() -> impl Signal<Item = DomBuilder<Element>> {
    let v = Mutable::new(42);

    let r = map_ref! {
        let value = v.signal() => move {
            new_html("span")
             .text(format!("The value is: {}", value).as_str())
        }
    };

    v.set(666);

    let bar = with_id(r, "test");
    with_id(bar, "bar")
}

#[cfg(test)]
mod test {

    use futures_signals::map_ref;
    use futures_signals::signal::{always, Mutable, SignalExt};
    use futures_util::StreamExt;

    use crate::components::bind;

    #[tokio::test]
    async fn test_bind() {
        let a = Mutable::new(42);
        let b = Mutable::new("hi");
        let c = Mutable::new(666);

        let out = bind(a.signal(), |input| {
            map_ref! {
                    let b2 = b.signal(),
                    let c2 = c.signal() => move {
                        return format!("{}:{}:{}", input, b2, c2);
                    }

            }
        });

        let mut strm = out.to_stream().take(2);

        assert_eq!(strm.next().await.unwrap(), "42:hi:666".to_string());

        c.set(555);

        assert_eq!(strm.next().await.unwrap(), "42:hi:555".to_string());
    }

    #[tokio::test]
    async fn dom_signal_test() {
        // Assert that we can use non copy or clone types (i.e dominators Dom or DomBuilder types)
        // with bind

        struct FakeDom {
            text: String,
        }

        let b = Mutable::new("hi");

        let ele = map_ref! {
            let b2 = b.signal() => move {
                FakeDom {
                    text: b2.to_string()                }
            }
        };

        let out = bind(ele, |v| always(v)).map(|db| db);
        let mut out = out.to_stream().take(2);

        let dom: FakeDom = out.next().await.unwrap();

        assert_eq!(dom.text, "hi");

        b.set("there");

        let dom: FakeDom = out.next().await.unwrap();

        assert_eq!(dom.text, "there");
    }
}
