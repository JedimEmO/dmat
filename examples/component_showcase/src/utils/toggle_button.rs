use dmat_components::components::{ButtonStyle, ButtonType};
use dominator::{clone, Dom};
use futures_signals::signal::Mutable;

/// Utility function to create buttons for toggling boolean mutables
pub fn toggle_button(value: &Mutable<bool>, label: &str) -> Dom {
    button!({
        .label(label)
        .click_handler(clone!(value => move |_| value.set(!value.get())))
        .button_type(ButtonType::Text)
        .style(ButtonStyle::Unimportant)
    })
}
