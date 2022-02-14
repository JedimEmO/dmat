use dominator::{clone, DomBuilder};
use futures::Stream;
use futures_signals::signal::Mutable;
use web_sys::Element;

use crate::utils::mixin::stream_handler_mixin;

/// Creates a mixin which will toggle the content of the value mutable for every item
/// in the source stream
#[inline]
pub fn stream_to_flipflop_mixin<A, TStream, TValue>(
    source_stream: TStream,
    value: &Mutable<bool>,
) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A>
where
    A: AsRef<Element>,
    TStream: Stream<Item = TValue> + Unpin + 'static,
{
    stream_handler_mixin(
        source_stream,
        clone!(value => move |_| {
            value.set(!value.get());
        }),
    )
}
