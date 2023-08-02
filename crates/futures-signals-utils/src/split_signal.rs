use futures_signals::signal::{channel, Signal, SignalExt};

/// All credit to Pauan https://github.com/Pauan for this function.
pub fn split_signal<S, A, F>(
    signal: S,
    initial_value: A,
    mut map: F,
) -> (impl Signal<Item = S::Item>, impl Signal<Item = A>)
where
    S: Signal,
    F: FnMut(&S::Item) -> A,
{
    let (sender, receiver) = channel(initial_value);

    let signal = signal.map(move |value| {
        let _ = sender.send(map(&value));
        value
    });

    (signal, receiver)
}
