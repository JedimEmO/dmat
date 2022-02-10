use dominator::DomBuilder;
use futures_signals::map_ref;
use futures_signals::signal::Signal;
use web_sys::Element;

/// Creates a DomBuilder<A> -> DomBuilder<A> lambda which will apply the
/// provided bool signal to the `-invalid` class signal of the builder
/// if the is_valid_signal parameter is some
#[inline]
pub fn with_invalid_signal<TSig, A: AsRef<Element>>(
    is_valid_signal: TSig,
) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A>
where
    TSig: Signal<Item = bool> + Unpin + 'static,
{
    move |d| {
        d.apply(move |inner_builder| {
            inner_builder.class_signal(
                "-invalid",
                map_ref!(let is_valid = is_valid_signal => !is_valid),
            )
        })
    }
}
