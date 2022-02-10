use dominator::DomBuilder;
use futures_signals::map_ref;
use futures_signals::signal::Signal;
use web_sys::Element;

/// Creates a DomBuilder<A> -> DomBuilder<A> lambda which will apply the
/// provided bool signal to the disabled signal of the builder
/// if the disabled_signal parameter is some
#[inline]
pub fn with_disabled_signal<TSig, A: AsRef<Element>>(
    disabled_signal: TSig,
) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A>
where
    TSig: Signal<Item = bool> + Unpin + 'static,
{
    move |d| {
        d.apply(move |inner_builder| {
            inner_builder.attribute_signal(
                "disabled",
                map_ref!(let is_disabled = disabled_signal=> {
                    match is_disabled {
                        true => Some("disabled"),
                        _ => None
                    }
                }),
            )
        })
    }
}
