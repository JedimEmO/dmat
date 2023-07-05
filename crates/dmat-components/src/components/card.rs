use crate::utils::component_signal::{ComponentSignal, NoSignal};
use crate::utils::mixin::ApplyMixin;
use dominator::{html, Dom, DomBuilder};
use futures_signals::signal::{always, Always};
use web_sys::HtmlElement;

#[inline]
pub fn card<TChildSignal: ComponentSignal>(props: CardProps<TChildSignal>) -> Dom {
    let apply = props.apply;
    let child = props.child;

    html!("div", {
        .class("dmat-card")
        .apply_if(apply.is_some(), |dom| dom.apply(apply.unwrap()))
        .apply_if(child.is_some(), |dom| dom.child_signal(child.unwrap()))
    })
}

#[macro_export]
macro_rules! card {
    ($($methods:tt)*) => {{
        let default_props =$crate::components::card::CardProps::new();
        let applied_props = dominator::apply_methods!(default_props, $($methods)*);
        $crate::components::card::card(applied_props)
    }};
}

pub struct CardProps<TChildSignal: ComponentSignal = NoSignal> {
    pub child: Option<TChildSignal>,
    pub apply: ApplyMixin,
}

impl CardProps {
    pub fn new() -> CardProps {
        Self {
            child: None,
            apply: None,
        }
    }
}

impl Default for CardProps {
    fn default() -> Self {
        Self::new()
    }
}

impl<TChildSignal: ComponentSignal> CardProps<TChildSignal> {
    #[inline]
    #[must_use]
    pub fn child(self, child: impl Into<Option<Dom>>) -> CardProps<Always<Option<Dom>>> {
        CardProps {
            child: Some(always(child.into())),
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn child_signal<TChildSignalNew: ComponentSignal>(
        self,
        child: TChildSignalNew,
    ) -> CardProps<TChildSignalNew> {
        CardProps {
            child: Some(child),
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn apply<F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static>(
        mut self,
        apply: F,
    ) -> Self {
        self.apply = Some(Box::new(apply));
        self
    }
}
