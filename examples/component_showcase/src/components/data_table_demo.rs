use dominator::{clone, html, Dom};
use futures_signals::signal::always;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use wasm_bindgen::__rt::std::rc::Rc;

use dmat_components::components::layouts::ContentBlockProps;
use dmat_components::components::TitleProps;
use dmat_components::components::{data_table, DataTableProps};

pub fn data_table_demo() -> Dom {
    let data: Rc<MutableVec<usize>> = Rc::new(MutableVec::new_with_values((0..10).collect()));
    let current_top = Mutable::new(0);
    let shared_data = Mutable::new("".to_string());

    let table = data_table(
        DataTableProps::new(
            data.clone(),
            clone!(shared_data => move |v| {
                let input_test_value = Mutable::new("".to_string());

                html!("tr", {
                    .children(&mut [
                        html!("td", {
                        .text(format!("{}", v).as_str())
                        }),
                        html!("td", {
                            .child(text_field!( {
                                .label("Data per row")
                                .value(input_test_value)
                            }).0)
                        }),
                        html!("td", {
                            .child(text_field!({
                                .label("Shared data")
                                .value(shared_data.clone())
                            }).0)
                        })
                    ])
                })
            }),
        )
        .headers(vec![
            "Column 1".to_string(),
            "Column 2".to_string(),
            "Column 3".to_string(),
        ])
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

    container!(|d| d.child(content_block!(
        ContentBlockProps {
            title_section: Some(title!(TitleProps {
                header_text_signal: always("Data table with pagination support".to_string()),
                sub_header_text_signal: always(None)
            })),
            media_section: Some(table),
            supporting_section: None,
            footer_section: None
        },
        |v| v.class("demo-card")
    ),))
}
