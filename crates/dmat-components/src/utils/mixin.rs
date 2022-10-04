use dominator::DomBuilder;
use futures::Stream;
use futures::StreamExt;
use web_sys::Element;

#[inline]
pub fn no_mixin<A: AsRef<Element>>(_dom_builder: DomBuilder<A>) -> DomBuilder<A> {
    _dom_builder
}

#[inline]
pub fn mixin_id<A: AsRef<Element>>() -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A> {
    no_mixin::<A>
}

#[inline]
pub fn id_attribute_mixin<T: AsRef<str> + 'static, A: AsRef<Element>>(
    id: T,
) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A> {
    move |d: DomBuilder<A>| d.attr("id", id.as_ref())
}

#[inline]
pub fn stream_handler_mixin<A, TStream, F, T>(
    input_stream: TStream,
    cb: F,
) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A>
where
    A: AsRef<Element>,
    TStream: Stream<Item = T> + Unpin + 'static,
    F: Fn(T) + 'static,
{
    move |d| {
        d.future(input_stream.for_each(move |item| {
            cb(item);
            async {}
        }))
    }
}
