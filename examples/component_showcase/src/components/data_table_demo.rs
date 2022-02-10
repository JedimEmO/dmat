use dominator::{clone, html, Dom};
use dominator_material::components::input::input_props::InputProps;
use dominator_material::components::ScrimProps;
use dominator_material::components::TextFieldProps;
use dominator_material::components::{data_table, CardProps, DataTableProps};
use futures_signals::map_ref;
use futures_signals::signal::always;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use wasm_bindgen::__rt::std::rc::Rc;

pub fn data_table_demo() -> Dom {
    let data: Rc<MutableVec<usize>> = Rc::new(MutableVec::new_with_values((0..10).collect()));
    let current_top = Mutable::new(0);

    let table = data_table(
        DataTableProps::new(data.clone(), |v, visible| {
            let input_test_value = Mutable::new("".to_string());

            html!("tr", {
                .children(&mut [
                    html!("td", {
                    .text(format!("{}", v).as_str())
                    }),
                    html!("td", {
                        .child_signal(visible.signal_ref(move |v| {
                            Some(if *v {
                                text_field!(TextFieldProps {
                                    claim_focus: false,
                                    input_props: InputProps{
                                        label: always(Some("With dynamic help text".to_string())),
                                        value: input_test_value.clone(),
                                        is_valid: always(true),
                                        assistive_text_signal: map_ref!(let cur_val = input_test_value.signal_cloned() =>
                                            Some(format!("Assistive text - {}", cur_val))),
                                        error_text_signal: always(None),
                                        disabled_signal: always(false)
                                    }
                                }).0
                            } else {
                                text!("hidden")
                            })
                        }))
                    })
                ])
            })
        })
        .headers(vec!["Column 1".to_string(), "Column 2".to_string()])
        .page_meta(
            Mutable::new(10),
            Mutable::new(100000),
            current_top.clone(),
            clone!(data, current_top => move |v, w| {
                data.lock_mut().replace_cloned((v..(v+w)).collect());
                current_top.replace(v);
            }),
            Some(vec![10, 20, 50, 100, 1000]),
        ),
    );

    card!(
        CardProps::new()
            .with_title(
                "Data table with pagination",
                Some("Page change triggers data regeneration"),
            )
            .body(table),
        |v| v.class("demo-card")
    )
}
