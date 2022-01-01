use dominator::{Dom, DomBuilder};
use futures_signals::signal::{always, Always, Signal, SignalExt};
use std::iter::{once, Once};
use web_sys::Element;

pub fn once_cmp(c: DomBuilder<Element>) -> Once<Always<Option<DomBuilder<Element>>>> {
    once(always(Some(c)))
}

pub fn builder_to_dom_signal<T: Signal<Item = DomBuilder<Element>> + Unpin>(
    input: T,
) -> impl Signal<Item = Option<Dom>> {
    input.map(|builder| Some(builder.into_dom()))
}

pub struct DomOption(pub Option<Dom>);

pub struct ComponentSignal(pub Box<dyn Signal<Item = Option<Dom>> + Unpin>);

impl ComponentSignal {
    pub fn from_signal<T: Signal<Item = U> + Unpin + 'static, U>(sig: T) -> Self
    where
        U: Into<DomOption>,
    {
        Self(Box::new(sig.map(|e| e.into().0)))
    }
}

impl From<Dom> for DomOption {
    fn from(v: Dom) -> Self {
        DomOption(Some(v))
    }
}

impl From<Option<Dom>> for DomOption {
    fn from(v: Option<Dom>) -> Self {
        DomOption(v)
    }
}

impl From<DomBuilder<Element>> for DomOption {
    fn from(v: DomBuilder<Element>) -> Self {
        DomOption(Some(v.into_dom()))
    }
}

impl Into<ComponentSignal> for Dom {
    fn into(self) -> ComponentSignal {
        ComponentSignal {
            0: Box::new(always(Some(self))),
        }
    }
}

impl Into<ComponentSignal> for DomBuilder<Element> {
    fn into(self) -> ComponentSignal {
        ComponentSignal {
            0: Box::new(always(Some(self.into_dom()))),
        }
    }
}
