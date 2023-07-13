use dominator::{html, Dom};
use futures_signals::signal_vec::{SignalVecExt};

#[component(render_fn = list)]
pub struct List {
    #[signal_vec]
    rows: Dom,
}

/// Renders a list of items.
///
/// # Examples
///
/// ```rust,no_run
/// use dominator::html;
/// use futures_signals::signal_vec::{MutableVec, SignalVecExt};
/// use dmat_components::components::list;
/// use dmat_components::list;
/// use dmat_components::components::list::*;
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
pub fn list(
    props: impl ListPropsTrait + 'static
) -> Dom {
    let ListProps {
        rows,
        apply,
    } = props.take();

    html!("ul", {
        .class("dmat-list")
        .apply_if(apply.is_some(), |dom| dom.apply(apply.unwrap()))
        .apply_if(rows.is_some(), |dom| {
            dom.children_signal_vec(rows.unwrap().map(child_row))
        })
    })
}

fn child_row(dom: Dom) -> Dom {
    html!("li", {
        .class("dmat-list-item")
        .child(dom)
    })
}
