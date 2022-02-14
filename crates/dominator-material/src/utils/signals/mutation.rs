use dominator::{clone, DomBuilder};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use web_sys::Element;

pub fn store_signal_value_mixin<A, TValue, TSignal>(
    source_signal: TSignal,
    target_mutable: &Mutable<TValue>,
) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A>
where
    A: AsRef<Element>,
    TValue: 'static,
    TSignal: Signal<Item = TValue> + 'static,
{
    clone!(target_mutable => move |d| {
        d.future(source_signal.for_each(clone!(target_mutable => move |v| {
            target_mutable.set(v);
            async {}
        })))
    })
}

pub fn store_signal_value_opt_mixin<A, TValue, TSignal>(
    source_signal: TSignal,
    target_mutable: &Mutable<TValue>,
) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A>
where
    A: AsRef<Element>,
    TValue: 'static,
    TSignal: Signal<Item = Option<TValue>> + 'static,
{
    clone!(target_mutable =>move |d| {
        d.future(
            source_signal
                .for_each(clone!(target_mutable => move |v| {
                    if let Some(value) = v {
                        target_mutable.set(value);
                    }
                    async {}
            })),
        )
    })
}
