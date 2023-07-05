use std::str::FromStr;
use std::sync::{Arc, RwLock};
use futures_signals::signal::{Always, always, Map, Mutable, MutableSignal, MutableSignalCloned, Signal};
use crate::futures_signals::signal::SignalExt;

pub struct InputProps<
    TLabelSignal: Signal<Item = Option<String>>,
    TValidSignal: Signal<Item = bool>,
    TAssistiveTextSignal: Signal<Item = Option<String>>,
    TErrorTextSignal: Signal<Item = Option<String>>,
    TDisabledSignal: Signal<Item = bool>
> {
    pub label: Option<TLabelSignal>,
    pub value: Mutable<String>,
    pub is_valid: Option<TValidSignal>,
    pub assistive_text_signal: Option<TAssistiveTextSignal>,
    pub error_text_signal: Option<TErrorTextSignal>,
    pub disabled_signal: Option<TDisabledSignal>,
}

impl<
    TLabelSignal: Signal<Item = Option<String>>,
    TValidSignal: Signal<Item = bool>,
    TAssistiveTextSignal: Signal<Item = Option<String>>,
    TErrorTextSignal: Signal<Item = Option<String>>,
    TDisabledSignal: Signal<Item = bool>
> InputProps<TLabelSignal, TValidSignal, TAssistiveTextSignal, TErrorTextSignal, TDisabledSignal> {
    #[inline]
    #[must_use]
    pub fn label<T: AsRef<str>>(self, label: T) -> InputProps<Always<Option<String>>, TValidSignal, TAssistiveTextSignal, TErrorTextSignal, TDisabledSignal>  {
        InputProps {
            label: Some(always(Some(label.as_ref().to_string()))),
            value: self.value,
            is_valid: self.is_valid,
            assistive_text_signal: self.assistive_text_signal,
            error_text_signal: self.error_text_signal,
            disabled_signal: self.disabled_signal,
        }
    }

    #[inline]
    #[must_use]
    pub fn label_signal<TNewLabelSignal: Signal<Item=Option<String>>+Unpin + 'static>(self, label: TNewLabelSignal) -> InputProps<TNewLabelSignal, TValidSignal, TAssistiveTextSignal, TErrorTextSignal, TDisabledSignal> {
        InputProps {
            label: Some(label),
            value: self.value,
            is_valid: self.is_valid,
            assistive_text_signal: self.assistive_text_signal,
            error_text_signal: self.error_text_signal,
            disabled_signal: self.disabled_signal,
        }
    }

    #[inline]
    #[must_use]
    pub fn assistive_text<T: AsRef<str>>(self, assistive_text: Option<T>) -> InputProps<TLabelSignal, TValidSignal, Always<Option<String>>, TErrorTextSignal, TDisabledSignal> {
        InputProps {
            label: self.label,
            value: self.value,
            is_valid: self.is_valid,
            assistive_text_signal: Some(always(assistive_text.map(|v| v.as_ref().to_string()))),
            error_text_signal: self.error_text_signal,
            disabled_signal: self.disabled_signal,
        }
    }

    #[inline]
    #[must_use]
    pub fn assistive_text_signal<TNewAssistiveTextSignal: Signal<Item=Option<String>>+Unpin + 'static>(self, assistive_text_signal: TNewAssistiveTextSignal) -> InputProps<TLabelSignal, TValidSignal, TNewAssistiveTextSignal, TErrorTextSignal, TDisabledSignal> {
        InputProps {
            label: self.label,
            value: self.value,
            is_valid: self.is_valid,
            assistive_text_signal: Some(assistive_text_signal),
            error_text_signal: self.error_text_signal,
            disabled_signal: self.disabled_signal,
        }
    }

    #[inline]
    #[must_use]
    pub fn error_text<T: AsRef<str>>(self, error_text: Option<T>) -> InputProps<TLabelSignal, TValidSignal, TAssistiveTextSignal, Always<Option<String>>, TDisabledSignal> {
        InputProps {
            label: self.label,
            value: self.value,
            is_valid: self.is_valid,
            assistive_text_signal: self.assistive_text_signal,
            error_text_signal: Some(always(error_text.map(|v| v.as_ref().to_string()))),
            disabled_signal: self.disabled_signal,
        }
    }

    #[inline]
    #[must_use]
    pub fn error_text_signal<TNewErrorTextSignal: Signal<Item=Option<String>>+Unpin + 'static>(self, error_text_signal: TNewErrorTextSignal) -> InputProps<TLabelSignal, TValidSignal, TAssistiveTextSignal, TNewErrorTextSignal, TDisabledSignal> {
        InputProps {
            label: self.label,
            value: self.value,
            is_valid: self.is_valid,
            assistive_text_signal: self.assistive_text_signal,
            error_text_signal: Some(error_text_signal),
            disabled_signal: self.disabled_signal,
        }
    }

    #[inline]
    #[must_use]
    pub fn disabled(self, disabled: bool) -> InputProps<TLabelSignal, TValidSignal, TAssistiveTextSignal, TErrorTextSignal, Always<bool>> {
        InputProps {
            label: self.label,
            value: self.value,
            is_valid: self.is_valid,
            assistive_text_signal: self.assistive_text_signal,
            error_text_signal: self.error_text_signal,
            disabled_signal: Some(always(disabled)),
        }
    }

    #[inline]
    #[must_use]
    pub fn disabled_signal<TNewDisabledSignal: Signal<Item=bool>+Unpin + 'static>(self, disabled_signal: TNewDisabledSignal) -> InputProps<TLabelSignal, TValidSignal, TAssistiveTextSignal, TErrorTextSignal, TNewDisabledSignal> {
        InputProps {
            label: self.label,
            value: self.value,
            is_valid: self.is_valid,
            assistive_text_signal: self.assistive_text_signal,
            error_text_signal: self.error_text_signal,
            disabled_signal: Some(disabled_signal),
        }
    }

    #[inline]
    #[must_use]
    pub fn is_valid(self, is_valid: bool) -> InputProps<TLabelSignal, Always<bool>, TAssistiveTextSignal, TErrorTextSignal, TDisabledSignal> {
        InputProps {
            label: self.label,
            value: self.value,
            is_valid: Some(always(is_valid)),
            assistive_text_signal: self.assistive_text_signal,
            error_text_signal: self.error_text_signal,
            disabled_signal: self.disabled_signal,
        }
    }

    #[inline]
    #[must_use]
    pub fn is_valid_signal<TNewIsValidSignal: Signal<Item=bool>+Unpin + 'static>(self, is_valid_signal: TNewIsValidSignal) -> InputProps<TLabelSignal, TNewIsValidSignal, TAssistiveTextSignal, TErrorTextSignal, TDisabledSignal> {
        InputProps {
            label: self.label,
            value: self.value,
            is_valid: Some(is_valid_signal),
            assistive_text_signal: self.assistive_text_signal,
            error_text_signal: self.error_text_signal,
            disabled_signal: self.disabled_signal,
        }
    }
}
