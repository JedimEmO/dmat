use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::always;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;

use dmat_components::components::input::input_props::InputProps;
use dmat_components::components::input::SelectProps;
use dmat_components::components::input::{ComboBoxProps, SwitchProps};
use dmat_components::components::list::*;
use dmat_components::utils::signals::stream_flipflop::stream_to_flipflop_mixin;

pub fn input_demo() -> Dom {
    let value = Mutable::new("".to_string());
    container!(|d| {
        d.children(&mut [
            text_input_demo(&value),
            combo_box_demo(&value),
            switch_demo(),
        ])
    })
}

fn switch_demo() -> Dom {
    let state = Mutable::new(true);
    let state2 = Mutable::new(true);
    let state3 = Mutable::new(false);

    let (sw, sw_clicks) = switch!(SwitchProps {
        state_signal: clone!(state => move || Box::new(state.signal_cloned())),
        disabled_signal: always(false)
    });

    let (sw_disabled_on, sw_clicks2) = switch!(SwitchProps {
        state_signal: clone!(state2 => move || Box::new(state2.signal_cloned())),
        disabled_signal: always(true)
    });

    let (sw_disabled_off, sw_clicks3) = switch!(SwitchProps {
        state_signal: clone!(state3 => move || Box::new(state3.signal_cloned())),
        disabled_signal: always(true)
    });

    card!({
        .child(list!({
            .rows([
                html!("div", { .children(&mut [text!("Switch: "), sw])}),
                html!("div", { .children(&mut [text!("Switch disabled, on: "), sw_disabled_on])}),
                html!("div", { .children(&mut [text!("Switch disabled, off: "), sw_disabled_off])}),
            ])
        }))
        .apply(move |b| {
            let b = b.class("switch-demo-list");
            let b = stream_to_flipflop_mixin(sw_clicks.toggle_stream, &state)(b);
            let b = stream_to_flipflop_mixin(sw_clicks2.toggle_stream, &state2)(b);
            stream_to_flipflop_mixin(sw_clicks3.toggle_stream, &state3)(b)
        })
    })
}

fn combo_box_demo(value: &Mutable<String>) -> Dom {
    card!({
        .child(list!({
            .rows([
                text!("Selection"),
                combo_box!(ComboBoxProps {
                    options: MutableVec::new_with_values(vec![
                        "Banana".to_string(),
                        "Orange".to_string(),
                        "Apple".to_string()
                    ]),
                    data_list_id: "demo-list-a".into(),
                    input_props: InputProps {
                        label: Some(always(Some("Oranges are the best".to_string()))),
                        value: value.clone(),
                        is_valid: Some(value.signal_ref(|v| v == "Orange")),
                        assistive_text_signal: Some(always(None)),
                        error_text_signal: Some(always(None)),
                        disabled_signal: Some(always(false))
                    }
                }),
                combo_box!(ComboBoxProps {
                    options: MutableVec::new_with_values(vec![
                        "Banana".to_string(),
                        "Orange".to_string(),
                        "Apple".to_string()
                    ]),
                    data_list_id: "demo-list-a".into(),
                    input_props: InputProps {
                        label: Some(always(Some("Oranges are the best".to_string()))),
                        value: value.clone(),
                        is_valid: Some(value.signal_ref(|v| v == "Orange")),
                        assistive_text_signal: Some(always(None)),
                        error_text_signal: Some(always(Some("With assistive/error text signal".to_string()))),
                        disabled_signal: Some(always(false))
                    }
                }),
                select!(SelectProps {
                    data_list_id: "data-list-b".to_string(),
                    options: MutableVec::new_with_values(vec![
                        "Banana".to_string(),
                        "Orange".to_string(),
                        "Apple".to_string()
                    ]),
                    input_props: InputProps {
                        label: Some(always(Some("Pick one".to_string()))),
                        value: value.clone(),
                        is_valid: Some(always(true)),
                        assistive_text_signal: Some(always(None)),
                        error_text_signal: Some(always(None)),
                        disabled_signal: Some(always(false))
                    }
                }),
                select!(SelectProps {
                    data_list_id: "data-list-c".to_string(),
                    options: MutableVec::new_with_values(vec![
                        "Banana".to_string(),
                        "Orange".to_string(),
                        "Apple".to_string()
                    ]),
                    input_props: InputProps {
                        label: Some(always(Some("select with assistive text".to_string()))),
                        value: value.clone(),
                        is_valid: Some(value.signal_ref(|v| v == "Banana")),
                        assistive_text_signal: Some(always(Some("This one likes Bananas".to_string()))),
                        error_text_signal: Some(always(None)),
                        disabled_signal: Some(always(false))
                    }
                }),
            ])
        }))
    })
}

fn text_input_demo(value: &Mutable<String>) -> Dom {
    card!({
        .child( list!({
            .rows([
                text!("Text field"),
                html!("div", {
                    .children(&mut [
                        text_field!({
                            .claim_focus()
                            .label("With dynamic help text")
                            .value(value.clone())
                            .assistive_text_signal(map_ref!(let cur_val = value.signal_cloned() =>
                                Some(format!("Assistive text - {}", cur_val))))
                        }).0
                    ])
                }),
                html!("div", {
                    .children(&mut [
                        text_field!({
                            .label("With error text")
                            .value(value.clone())
                            .is_valid_signal(value.signal_ref(|v| v.contains("foobar")))
                            .assistive_text_signal(map_ref!(let cur_val = value.signal_cloned() =>
                                Some(format!("Assistive text - {}", cur_val))))
                            .error_text_signal(always(Some("Accepts string containing `foobar`".to_string())))
                        }).0
                    ])
                }),
                html!("div", {
                    .children(&mut [
                        text_field!({
                            .label("Always invalid")
                            .value(value.clone())
                        }).0
                    ])
                }),
            ])
        }))
        .apply(|v| v.class("demo-card"))
    })
}
