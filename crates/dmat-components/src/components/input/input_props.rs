use futures_signals::signal::{Mutable, Signal};

pub struct InputProps<
    TLabelSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TValidSignal: Signal<Item = bool> + Unpin + 'static,
    TAssistiveTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TErrorTextSignal: Signal<Item = Option<String>> + Unpin + 'static,
    TDisabledSignal: Signal<Item = bool> + Unpin + 'static,
> {
    pub label: TLabelSignal,
    pub value: Mutable<String>,
    pub is_valid: TValidSignal,
    pub assistive_text_signal: TAssistiveTextSignal,
    pub error_text_signal: TErrorTextSignal,
    pub disabled_signal: TDisabledSignal,
}
