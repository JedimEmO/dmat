use std::str::FromStr;

use dominator::{clone, events, html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::MutableVec;
use futures_signals::signal_vec::SignalVecExt;
use wasm_bindgen::__rt::core::time::Duration;
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::components::{progress_indicator, ProgressIndicatorIterations};

enum RenderFunc<T: 'static> {
    Row(Rc<dyn Fn(&T) -> Dom>),
    Cells(Rc<dyn Fn(&T) -> Vec<Dom>>),
}

#[derive(Clone)]
struct PageMeta {
    page_size: Mutable<usize>,
    total_data_count: Mutable<usize>,
    current_top: Mutable<usize>,
    on_request_data: Rc<dyn Fn(usize, usize)>,
    allowed_page_sizes: Option<Vec<usize>>,
}

pub struct DataTableProps<T: Clone + 'static> {
    data: Rc<MutableVec<T>>,
    page_meta: Option<PageMeta>,
    headers: Option<Vec<String>>,
    render_func: RenderFunc<T>,
    is_loading: Mutable<bool>,
}

impl<T: Clone + 'static> DataTableProps<T> {
    pub fn new(data: Rc<MutableVec<T>>) -> DataTableProps<T> {
        DataTableProps {
            data,
            page_meta: None,
            headers: None,
            render_func: RenderFunc::Row(Rc::new(
                |_| html!("span", {.text("No row rendering function provided ")}),
            )),
            is_loading: Default::default(),
        }
    }

    #[inline]
    #[must_use]
    pub fn headers(mut self, headers: Vec<String>) -> Self {
        self.headers = Some(headers);
        self
    }

    #[inline]
    #[must_use]
    pub fn row_render_func<F: 'static>(mut self, func: F) -> Self
    where
        F: Fn(&T) -> Dom,
    {
        self.render_func = RenderFunc::Row(Rc::new(func));
        self
    }

    #[inline]
    #[must_use]
    pub fn cell_render_func<F: 'static>(mut self, func: F) -> Self
    where
        F: Fn(&T) -> Vec<Dom>,
    {
        self.render_func = RenderFunc::Cells(Rc::new(func));
        self
    }

    #[inline]
    #[must_use]
    pub fn page_meta<F>(
        mut self,
        page_size: Mutable<usize>,
        total_data_count: Mutable<usize>,
        current_top: Mutable<usize>,
        on_page_change: F,
        allowed_page_sizes: Option<Vec<usize>>,
    ) -> Self
    where
        F: Fn(usize, usize) + 'static,
    {
        self.page_meta = Some(PageMeta {
            page_size,
            current_top,
            total_data_count,
            allowed_page_sizes,
            on_request_data: Rc::new(on_page_change),
        });

        self
    }
}

#[inline]
pub fn data_table<T: Clone + 'static>(props: DataTableProps<T>) -> Dom {
    let data_table = Rc::new(props);

    let heads = match &data_table.headers {
        Some(headers) => html!("tr", {
        .children(headers.iter().map(|th| html!("th", {
                .text(th)
            })).collect::<Vec<Dom>>().as_mut_slice())
        }),
        _ => html!("tr"),
    };

    let rows = clone!(data_table => data_table.data.signal_vec_cloned().map(move |val| {
        let loading_toggle_lambda = Closure::wrap(Box::new(clone!(data_table => move || {
            data_table.is_loading.replace(false);
        })) as Box<dyn Fn()>);

        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                loading_toggle_lambda.as_ref().unchecked_ref(),
                500,
            )
            .unwrap();

        loading_toggle_lambda.forget();

        match &data_table.render_func {
            RenderFunc::Row(render) => render(&val),
            RenderFunc::Cells(render) => {
                html!("tr", {
                    .children(render(&val).as_mut_slice())
                })
            }
        }
    }));

    let foot = match &data_table.page_meta {
        Some(meta) => table_pagination(meta, data_table.is_loading.clone()),
        _ => html!("tfoot"),
    };

    html!("table", {
        .class("dmat-table")
        .class_signal("--loading", data_table.is_loading.signal_cloned())
        .children(&mut [
            html!("thead", {
                .children(&mut [
                    heads,
                    html!("tr", {
                        .class("loading-row")
                        .child(html!("th", {
                            .attribute("colspan", "100")
                            .child_signal(data_table.is_loading.signal_cloned().map(|loading| {
                                match loading {
                                    true => Some(progress_indicator(Duration::from_millis(500), ProgressIndicatorIterations::Count(1))),
                                    _ => None
                                }
                            }))
                        }))
                    })
                ])
            }),
            html!("tbody", {
                .children_signal_vec(rows)
            }),
            foot
        ])
    })
}

fn table_pagination(meta: &PageMeta, loading: Mutable<bool>) -> Dom {
    let mut pagination_controls = vec![
        html!("span", {
            .text_signal(meta.current_top.signal_ref(|cur_page| format!("{}-", cur_page + 1)))
        }),
        html!("span", {
            .text_signal(meta.current_top.signal_ref(clone!(meta => move |cur_page| format!("{}", cur_page + meta.page_size.get()))))
        }),
        html!("span", {
            .text_signal(meta.total_data_count.signal_ref(|count| format!(" of {}", count)))
        }),
        html!("button", {
            .text("<<")
            .event(clone!(meta, loading => move |_: events::Click |{
                loading.replace(true);
                (meta.on_request_data)(0, meta.page_size.get());
            }))
            .property_signal("disabled", meta.current_top.signal_cloned().map(|v| v == 0))
            .class("dmat-pagination-button")
        }),
        html!("button", {
            .text("<")
            .event(clone!(meta, loading => move |_: events::Click |{
                loading.replace(true);
                let target_top = std::cmp::max(meta.current_top.get() as i32 - meta.page_size.get() as  i32, 0) as usize;
                (meta.on_request_data)(target_top, meta.page_size.get());
            }))
            .property_signal("disabled", meta.current_top.signal_cloned().map(|v| v == 0))
            .class("dmat-pagination-button")
        }),
        html!("button", {
            .text(">")
            .event(clone!(meta, loading => move |_: events::Click |{
                loading.replace(true);
                let count = meta.total_data_count.get();
                let target_top = std::cmp::min(meta.current_top.get() + meta.page_size.get(), count);
                (meta.on_request_data)(target_top, meta.page_size.get());
            }))
            .property_signal("disabled", meta.current_top.signal_cloned().map(clone!(meta => move |v| v + meta.page_size.get() >= meta.total_data_count.get())))
            .class("dmat-pagination-button")
        }),
        html!("button", {
            .text(">>")
            .event(clone!(meta, loading => move |_: events::Click |{
                loading.replace(true);
                let count = meta.total_data_count.get();
                let target_top = (count / meta.page_size.get()) * meta.page_size.get();
                (meta.on_request_data)(target_top, meta.page_size.get());
            }))
            .property_signal("disabled", meta.current_top.signal_cloned().map(clone!(meta => move |v| v + meta.page_size.get() >= meta.total_data_count.get())))
            .class("dmat-pagination-button")
        }),
    ];

    if let Some(allowed_pages) = &meta.allowed_page_sizes {
        let ps = meta.page_size.clone();
        let fetcher = meta.on_request_data.clone();
        let top = meta.current_top.clone();
        let pages = allowed_pages.clone();

        pagination_controls.insert(
            0,
            html!("select", {
                .event(clone!(ps, loading => move |evt: events::Change| {
                    if let Some(select) = evt.target() {
                        if let Some(select) = select.dyn_ref::<web_sys::HtmlSelectElement>() {
                            let page = usize::from_str(select.value().as_str()).unwrap();
                            loading.replace(true);
                            ps.replace(page);
                            fetcher(top.get(), page);
                        }
                    }
                }))
                .children(pages.into_iter().map(|page| html!("option", {
                    .text(format!("{}", page).as_str())
                    .property("value", format!("{}", page).as_str())
                })))
            }),
        )
    }

    html!("tfoot", {
        .child(html!("tr", {
            .child(html!("td", {
                .class("dmat-pagination")
                .attribute("colspan", "100")
                .child(html!("div", {
                    .children(pagination_controls.as_mut_slice())
                }))
            }))
        }))
    })
}
