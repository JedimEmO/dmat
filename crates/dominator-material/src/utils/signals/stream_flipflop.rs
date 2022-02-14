use futures::{Stream, StreamExt};
use futures_signals::signal::{from_stream, Signal, SignalExt};

/// Converts the source stream into a signal which outputs a boolean value that is toggled for each item in the stream.
pub fn stream_to_flipflop_signal<TStream, TValue>(
    source_stream: TStream,
    initial_value: bool,
) -> impl Signal<Item = bool>
where
    TStream: Stream<Item = TValue>,
{
    let mut value = initial_value;

    from_stream(source_stream.map(move |_| {
        value = !value;
        value
    }))
    .map(move |value| value.unwrap_or(initial_value))
}
