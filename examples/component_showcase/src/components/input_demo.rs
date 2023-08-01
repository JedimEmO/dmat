use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::always;
use futures_signals::signal::Mutable;

use dmat_components::components::input::combo_box::*;
use dmat_components::components::layouts::*;

use dmat_components::components::input::select::*;
use dmat_components::components::input::switch::*;
use dmat_components::components::input::validation_result::ValidationResult;
use dmat_components::components::input::value_adapters::mutable_t_value_adapter::MutableTValueAdapter;
use dmat_components::components::*;
use dmat_components::utils::signals::stream_flipflop::stream_to_flipflop_mixin;

pub fn input_demo() -> Dom {
    let value = Mutable::new("".to_string());
    container!({
        .children([
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
                combo_box!({
                    .options(make_select_options())
                    .data_list_id("demo-list-a".into())
                    .value_signal(value.signal_cloned())
                    .on_change(clone!(value => move |v| value.set(v)))
                    .is_valid_signal(value.signal_ref(|v| v == "Orange"))
                }),
                combo_box!({
                    .options(make_select_options())
                    .data_list_id("demo-list-b".into())
                    .label(Some(html!("span", { .text("Oranges are the best")})))
                    .value_signal(value.signal_cloned())
                    .on_change(clone!(value => move |v| value.set(v)))
                    .is_valid_signal(value.signal_ref(|v| v == "Orange"))
                    .error_text(Some(html!("span", {.text("Error text signal")})))
                }),
                select!({
                    .options(make_select_options())
                    .label(Some(html!("span", { .text("Pick one")})))
                    .value_signal(value.signal_cloned())
                    .on_change(clone!(value => move |v| value.set(v)))
                }),
                select!({
                    .options(make_select_options())
                    .label(Some(html!("span", { .text("select with assistive text")})))
                    .value_signal(value.signal_cloned())
                    .on_change(clone!(value => move |v| value.set(v)))
                    .is_valid_signal(value.signal_ref(|v| v == "Banana"))
                    .assistive_text(Some(html!("span", { .text("This one likes Bananas")})))
                }),
            ])
        }))
    })
}

fn text_input_demo(value: &Mutable<String>) -> Dom {
    let i32_value = Mutable::new(0);

    card!({
        .child( list!({
            .rows([
                text!("Text field"),
                html!("div", {
                    .children(&mut [
                        text_field!({
                            .claim_focus(true)
                            .label(Some(html!("span", { .text("With dynamic help text")})))
                            .value(MutableTValueAdapter::new_simple(value))
                            .assistive_text_signal(map_ref!(let cur_val = value.signal_cloned() =>
                                Some(html!("span", {
                                    .text(format!("Assistive text - {}", cur_val).as_str())
                                }))))
                        }).0
                    ])
                }),
                html!("div", {
                    .children(&mut [
                        text_field!({
                            .label(Some(html!("span", { .text("With error text")})))
                            .value(MutableTValueAdapter::new_simple(value))
                            .is_valid_signal(value.signal_ref(|v| v.contains("foobar")))
                            .assistive_text_signal(map_ref!(let cur_val = value.signal_cloned() =>
                                Some(html!("span", {
                                    .text(format!("Assistive text - {}", cur_val).as_str())
                                }))))
                            .error_text(Some(html!("span", { .text("Accepts string containing `foobar`")})))
                        }).0
                    ])
                }),
                html!("div", {
                    .children(&mut [
                        text_field!({
                            .label(Some(html!("span", { .text("Always invalid")})))
                            .value(MutableTValueAdapter::new_simple(value))
                            .is_valid(false)
                        }).0
                    ])
                }),
                html!("div", {
                    .children(&mut [
                        text_field!({
                            .label(Some(html!("span", { .text("Only accepts UPPERCASE characters")})))
                            .value(MutableTValueAdapter::new_with_sanitizer(value, |v| {
                                if v.chars().all(|c| c.is_uppercase()) {
                                    ValidationResult::Valid
                                } else {
                                    ValidationResult::Invalid{message: "Sanitized for uppercase characters".to_string()}
                                }
                            }))
                        }).0
                    ])
                }),

                html!("div", {
                    .children(&mut [
                        text_field!({
                            .label(Some(html!("span", { .text("Only accepts i32 values")})))
                            .value(MutableTValueAdapter::new_simple(&i32_value))
                            .assistive_text_signal(map_ref!(let cur_val = i32_value.signal_cloned() =>
                                Some(html!("span", {
                                    .text(format!("{}*2={}", cur_val, cur_val*2).as_str())
                                }))))
                        }).0
                    ])
                }),
            ])
        }))
        .apply(|v| v.class("demo-card"))
    })
}

fn make_select_options() -> Vec<SelectOption> {
    vec![
        SelectOption {
            value: "Banana".to_string(),
            display_text: "Banana".to_string(),
        },
        SelectOption {
            value: "Orange".to_string(),
            display_text: "Orange".to_string(),
        },
        SelectOption {
            value: "Apple".to_string(),
            display_text: "Nice red apples".to_string(),
        },
    ]
}
