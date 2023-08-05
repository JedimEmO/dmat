use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::Mutable;

use dmat_components::components::input::validation_result::ValidationResult;
use dmat_components::components::input::value_adapters::mutable_t_value_adapter::MutableTValueAdapter;
use dmat_components::components::input::*;
use dmat_components::components::*;

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

    card!({
        .child(list!({
            .items([
                html!("div", { .children(&mut[
                    html!("span", {.text("Switch: ")}),
                    switch!({
                        .state_signal(state.signal())
                        .click_handler(clone!(state => move |_| {
                            state.set(!state.get())
                        }))
                    })
                ])}),
                html!("div", { .children(&mut [
                    html!("span", {.text("Switch disabled, on: ")}),
                    switch!({
                        .state_signal(state2.signal())
                        .disabled(true)
                    })
                ])}),
                html!("div", { .children(&mut [
                    html!("span", {.text("Switch disabled, off: ")}),
                    switch!({
                        .state_signal(state3.signal())
                        .disabled(true)
                    })
                ])}),
            ])
        }))
    })
}

fn combo_box_demo(value: &Mutable<String>) -> Dom {
    card!({
        .child(list!({
            .items([
                html!("span", { .text("Selection")}),
                combo_box!({
                    .input_id(Some("demo-cbox-1".into()))
                    .label(Some(html!("span", { .text("Combo Box")})))
                    .options(make_select_options())
                    .data_list_id("demo-list-a".into())
                    .value_signal(value.signal_cloned())
                    .on_change(clone!(value => move |v| value.set(v)))
                    .is_valid_signal(value.signal_ref(|v| {
                        if v == "Orange" {
                            ValidationResult::Valid
                        } else {
                            ValidationResult::Invalid { message: "I want oranges!".to_string() }
                        }
                    }))
                }),
                select!({
                    .input_id(Some("demo-sel-1".into()))
                    .options(make_select_options())
                    .label(Some(html!("span", { .text("Select")})))
                    .value_signal(value.signal_cloned())
                    .on_change(clone!(value => move |v| value.set(v)))
                }),
                select!({
                    .input_id(Some("demo-sel-2".into()))
                    .options(make_select_options())
                    .label(Some(html!("span", { .text("Select with assistive text")})))
                    .value_signal(value.signal_cloned())
                    .on_change(clone!(value => move |v| value.set(v)))
                    .is_valid_signal(value.signal_ref(|v| {
                        if v == "Banana" {
                            ValidationResult::Valid
                        } else {
                            ValidationResult::Invalid { message: "I want bananas!".to_string() }
                        }
                    }))
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
            .items([
                html!("span", { .text("Text input")}),
                html!("div", {
                    .children(&mut [
                        text_field!({
                            .input_id(Some("demo-txt-1".into()))
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
                            .input_id(Some("demo-txt-2".into()))
                            .label(Some(html!("span", { .text("With error text")})))
                            .value(MutableTValueAdapter::new_simple(value))
                            .is_valid_signal(value.signal_ref(|v| {
                                if v.contains("foobar") {
                                    ValidationResult::Valid
                                } else {
                                    ValidationResult::Invalid { message: "Must contain foobar".to_string() }
                                }
                            }))
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
                            .input_id(Some("demo-txt-3".into()))
                            .label(Some(html!("span", { .text("Always invalid")})))
                            .value(MutableTValueAdapter::new_simple(value))
                            .is_valid(validation_result::ValidationResult::Invalid { message: "Always invalid".to_string() })
                        }).0
                    ])
                }),
                html!("div", {
                    .children(&mut [
                        text_field!({
                            .input_id(Some("demo-txt-4".into()))
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
                            .input_id(Some("demo-txt-5".into()))
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
