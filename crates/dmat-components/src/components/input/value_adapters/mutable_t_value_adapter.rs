use crate::components::input::validation_result::ValidationResult;
use crate::components::input::value_adapters::value_adapter::ValueAdapter;
use futures_signals::signal::{Map, Mutable, MutableSignalCloned, SignalExt};
use std::str::FromStr;

pub struct MutableTValueAdapter<
    T: FromStr + ToString + Clone = String,
    TSanitizer: Fn(String) -> ValidationResult = fn(String) -> ValidationResult,
> {
    pub value: Mutable<T>,
    pub sanitizer: Option<TSanitizer>,
}

pub struct MutableI32ValueAdapter<
    TSanitizer: Fn(i32) -> ValidationResult = fn(i32) -> ValidationResult,
> {
    pub value: Mutable<i32>,
    pub sanitizer: Option<TSanitizer>,
}

impl MutableTValueAdapter {
    pub fn new_simple<T: FromStr + ToString + Clone>(
        value: &Mutable<T>,
    ) -> MutableTValueAdapter<T> {
        MutableTValueAdapter {
            value: value.clone(),
            sanitizer: None,
        }
    }

    pub fn new_with_sanitizer<
        T: FromStr + ToString + Clone,
        TSanitizer: Fn(String) -> ValidationResult + 'static,
    >(
        value: &Mutable<T>,
        sanitizer: TSanitizer,
    ) -> MutableTValueAdapter<T, TSanitizer> {
        MutableTValueAdapter {
            value: value.clone(),
            sanitizer: Some(sanitizer),
        }
    }
}

impl<T: FromStr + ToString + Clone, TSanitizer: Fn(String) -> ValidationResult + 'static>
    ValueAdapter for MutableTValueAdapter<T, TSanitizer>
{
    type ValueSignal = Map<MutableSignalCloned<T>, fn(T) -> String>;

    fn get_value_signal(&self) -> Self::ValueSignal {
        self.value.signal_cloned().map(|value| value.to_string())
    }

    fn set_value(&self, value: String) -> ValidationResult {
        if let Ok(parsed_value) = T::from_str(value.as_str()) {
            let out_result = if let Some(sanitizer) = &self.sanitizer {
                sanitizer(value)
            } else {
                ValidationResult::Valid
            };

            if out_result.is_valid() {
                self.value.set(parsed_value);
                ValidationResult::Valid
            } else {
                out_result
            }
        } else {
            ValidationResult::Invalid {
                message: "Failed to parse value".to_string(),
            }
        }
    }
}

impl Default for MutableTValueAdapter {
    fn default() -> Self {
        MutableTValueAdapter {
            value: Mutable::new("".to_string()),
            sanitizer: None,
        }
    }
}
