use crate::components::input::validation_result::ValidationResult;
use futures_signals::signal::Signal;

pub trait ValueAdapter {
    type ValueSignal: Signal<Item = String>;
    fn get_value_signal(&self) -> Self::ValueSignal;
    fn set_value(&self, value: String) -> ValidationResult;
}
