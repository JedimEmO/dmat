use dominator::{clone, Dom};
use dominator_material::components::{ButtonContent, ButtonProps};
use futures_signals::signal::{always, Mutable};

/// Utility function to create buttons for toggling boolean mutables
pub fn toggle_button(value: &Mutable<bool>, label: &str) -> Dom {
    let props = ButtonProps {
        content: Some(ButtonContent::Label(label.to_string())),
        click_handler: clone!(value => move |_| value.set(!value.get())),
        button_type: Default::default(),
        style: Default::default(),
        disabled_signal: always(false),
    };

    button!(props)
}
