use dominator::DomBuilder;
use futures::Stream;
use futures::StreamExt;
use futures_signals::signal::Mutable;
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
pub fn with_id<T: AsRef<str> + 'static, A: AsRef<Element>>(
    id: T,
) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A> {
    move |d: DomBuilder<A>| d.attribute("id", id.as_ref())
}

/// Creates a mixin which will flip the value in `latched_value` for every element of `input_stream`
/// useful when dealing with event streams that are used for toggling, such as `scrim!()`
///
/// # Examples
///
/// ```no_run
/// use futures_signals::signal::Mutable;
/// use dominator::html;
/// use futures_signals::signal::SignalExt;
/// use dominator_material::utils::mixin::with_stream_flipflop;
///
/// let test_input = Mutable::new(false);
/// let test_output = Mutable::new(true);
///
/// let _ = html!("div", {
///   .apply(with_stream_flipflop(test_input.signal_cloned().to_stream(), test_output.clone()))
///   .text_signal(test_output.signal_ref(|v| if *v { "true" } else { "false" }))
/// });
/// ```
#[inline]
pub fn with_stream_flipflop<A, TStream, TInner>(
    mut input_stream: TStream,
    latched_value: Mutable<bool>,
) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A>
where
    A: AsRef<Element>,
    TStream: Stream<Item = TInner> + Unpin + 'static,
{
    move |d| {
        d.future(async move {
            while let Some(_) = input_stream.next().await {
                latched_value.set(!latched_value.get());
            }
        })
    }
}
