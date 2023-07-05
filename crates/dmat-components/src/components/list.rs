use crate::utils::mixin::ApplyMixin;
use dominator::{html, Dom, DomBuilder};
use futures_signals::signal_vec::{always, Always, SignalVec, SignalVecExt};
use web_sys::HtmlElement;

/// Renders a list of items.
///
/// # Examples
///
/// ```rust,no_run
/// use dominator::html;
/// use futures_signals::signal_vec::{MutableVec, SignalVecExt};
/// use dmat_components::components::list;
/// use dmat_components::list;
///
/// // Static list, items never change
/// let my_static_list = list!({.rows([
///     html!("span", {.text("Hello")}),
///     html!("span", {.text("World")}),
/// ])});
///
/// // Dynamic list, items change according to the content of the mutable vector
/// let my_items = MutableVec::new_with_values(vec!["Hello", "World"]);
/// let my_dynamic_list = list!({
///     .rows_signal_vec(my_items.signal_vec_cloned()
///         .map(|item| html!("span", { .text(item) })))
/// });
/// ```
#[inline]
pub fn list<TRowsSignalVec: SignalVec<Item = Dom> + Unpin + 'static>(
    props: ListProps<TRowsSignalVec>,
) -> Dom {
    let ListProps {
        rows_signal_vec,
        apply,
    } = props;

    html!("ul", {
        .class("dmat-list")
        .apply_if(apply.is_some(), |dom| dom.apply(apply.unwrap()))
        .apply_if(rows_signal_vec.is_some(), |dom| {
            dom.children_signal_vec(rows_signal_vec.unwrap().map(child_row))
        })
    })
}

fn child_row(dom: Dom) -> Dom {
    html!("li", {
        .class("dmat-list-item")
        .child(dom)
    })
}

pub struct ListProps<TRowsSignalVec: SignalVec<Item = Dom> = Always<Dom>> {
    rows_signal_vec: Option<TRowsSignalVec>,
    apply: ApplyMixin,
}

impl ListProps {
    pub fn new() -> ListProps {
        Self {
            rows_signal_vec: None,
            apply: None,
        }
    }
}

impl Default for ListProps {
    fn default() -> Self {
        Self::new()
    }
}

impl<TRowsSignalVec: SignalVec<Item = Dom>> ListProps<TRowsSignalVec> {
    #[inline]
    #[must_use]
    pub fn rows_signal_vec<TRowsSignalVecNew: SignalVec<Item = Dom>>(
        self,
        rows_signal_vec: TRowsSignalVecNew,
    ) -> ListProps<TRowsSignalVecNew> {
        ListProps {
            rows_signal_vec: Some(rows_signal_vec),
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn rows(self, rows: impl Into<Vec<Dom>>) -> ListProps<Always<Dom>> {
        ListProps {
            rows_signal_vec: Some(always(rows.into())),
            apply: self.apply,
        }
    }

    #[inline]
    #[must_use]
    pub fn apply(
        mut self,
        apply: impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static,
    ) -> Self {
        self.apply = Some(Box::new(apply));
        self
    }
}

#[macro_export]
macro_rules! list {
        ($($methods:tt)*) => {{
        let default_props =$crate::components::list::ListProps::new();
        let applied_props = dominator::apply_methods!(default_props, $($methods)*);
        $crate::components::list::list(applied_props)
    }};
}

#[inline]
pub fn static_list<TChildren, F>(children: TChildren, mixin: F) -> Dom
where
    TChildren: IntoIterator<Item = Dom>,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("ul", {
        .class("dmat-list")
        .class("-static")
        .apply(mixin)
        .children(children.into_iter().map(|child| {
             html!("li", {
                .class("dmat-list-item")
                .child(child)
            })
        }))
    })
}
