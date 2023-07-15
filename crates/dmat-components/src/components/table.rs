use std::time::Duration;

use dominator::{html, Dom, DomBuilder};
use futures_signals::signal::{Signal, SignalExt};
use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use web_sys::HtmlElement;

use crate::components::ProgressIndicatorIterations;

/// Renders a simple table. Allows for a loading state to be displayed.
#[component(render_fn = table)]
struct Table {
    #[signal_vec]
    #[default(Vec::new())]
    headers: Dom,

    #[signal_vec]
    #[default(Vec::new())]
    rows: Vec<Dom>,

    #[signal_vec]
    footer: Dom,

    #[signal]
    is_loading: bool,
}

pub fn table(props: impl TablePropsTrait + 'static) -> Dom {
    let TableProps {
        headers,
        rows,
        footer,
        is_loading,
        apply,
    } = props.take();

    let is_loading_bc = is_loading.map(|v| v.broadcast());

    html!("table", {
        .class("dmat-table")
        .apply_if(is_loading_bc.is_some(), |d| d.class_signal("--loading", is_loading_bc.clone().unwrap().signal()))
        .child(thead(headers))
        .apply(|b| loading_row(b, is_loading_bc.map(|v| v.signal())))
        .child(tbody(rows))
        .apply_if(footer.is_some(), |dom| dom.child(tfoot(footer.unwrap())))
        .apply_if(apply.is_some(), |dom| dom.apply(apply.unwrap()))
    })
}

fn thead(headers: impl SignalVec<Item = Dom> + 'static) -> Dom {
    html!("thead", {
        .child(html ! ("tr", {
            .children_signal_vec(headers.map( | header_cell | {
                html ! ("th", {
                    .child(header_cell)
                })
            }))
        }))
    })
}

fn tbody(rows: impl SignalVec<Item = Vec<Dom>> + 'static) -> Dom {
    html!("tbody", {
    .children_signal_vec(rows.map( | row_cells | {
        html ! ("tr", {
            .children(row_cells.into_iter().map( | cell | {
                html ! ("td", {
                .child(cell)
                })
            }).collect::< Vec < Dom > >().as_mut_slice())
        })
    }))
    })
}

fn tfoot(footer: impl SignalVec<Item = Dom> + 'static) -> Dom {
    html!("tfoot", {
        .child(html!("tr", {
            .children_signal_vec(footer.map(|f| html!("td", {
                .child(f)
            })))
        }))
    })
}

fn loading_row(
    dom_builder: DomBuilder<HtmlElement>,
    is_loading: Option<impl Signal<Item = bool> + 'static>,
) -> DomBuilder<HtmlElement> {
    dom_builder.apply_if(is_loading.is_some(), |dom| {
        dom.child_signal(is_loading.unwrap().map(|is_loading_now| {
            if is_loading_now {
                Some( html!("tr", {
                    .class("loading-row")
                    .child(html!("th", {
                        .attr("colspan", "10000")
                        .child(crate::progress_indicator ! (Duration::from_millis(500), ProgressIndicatorIterations::Count(1)))
                    }))
                }))
            } else {
                None
            }
        }))
    })
}
