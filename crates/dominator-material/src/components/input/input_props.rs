use futures_signals::signal::{Mutable, Signal};

pub struct InputProps {
    pub label: Option<Box<dyn Signal<Item = String> + Unpin>>,
    pub value: Mutable<String>,
    pub is_valid: Option<Box<dyn Signal<Item = bool> + Unpin>>,
    pub assistive_text_signal: Option<Box<dyn Signal<Item = Option<String>> + Unpin>>,
    pub error_text_signal: Option<Box<dyn Signal<Item = Option<String>> + Unpin>>,
}
