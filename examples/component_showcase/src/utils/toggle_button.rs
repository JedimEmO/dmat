use dmat_components::components::{ButtonContent, ButtonProps, ButtonStyle, ButtonType};
use dominator::{clone, Dom};
use futures_signals::signal::{always, Mutable};

/// Utility function to create buttons for toggling boolean mutables
pub fn toggle_button(value: &Mutable<bool>, label: &str) -> Dom {
    let props = ButtonProps {
        content: Some(ButtonContent::Label(label.to_string())),
        click_handler: clone!(value => move |_| value.set(!value.get())),
        button_type: ButtonType::Text,
        style: ButtonStyle::Unimportant,
        disabled_signal: always(false),
    };

    button!(props)
}
