use dominator::{clone, html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use lipsum::lipsum;

use dmat_components::components::layouts::*;
use dmat_components::components::*;

use crate::utils::toggle_button::toggle_button;

#[inline]
pub fn navigation_drawers_demo() -> Dom {
    list!({
        .rows([
        container!({.children([
            card!({
                .child(content_block!({
                    .title_section(Some(title!({
                        .header_text("Retracting modal drawer".to_string())
                    })))
                    .media_section(Some(list!({
                            .rows([html!("div", {
                            .class("navigation-drawer-demo")
                            .child(retracting(true))
                        })])
                    })))
                }))
                .apply(|d| d.class("drawer-demo-card").style("height", "350px"))
            }),
            card!({
                .child( content_block!({
                    .title_section(Some(title!({
                        .header_text("Retracting non-modal drawer".to_string())
                    })))
                    .media_section(Some(list!({
                        .rows([
                                html!("div", {
                                .class("navigation-drawer-demo")
                                .child(retracting(false))
                            })
                        ])
                    })))
                }))
                .apply(|d| d.class("drawer-demo-card").style("height", "350px"))
            }),
        ])}),
        container!({.children([
            card!({
                .child(content_block!({
                    .title_section(Some(title!({
                        .header_text("Modal toggled".to_string())
                    })))
                    .media_section(Some(list!({
                            .rows([
                                html!("div", {
                                    .class("navigation-drawer-demo")
                                    .apply(|d| {
                                        let toggled = toggled(true);
                                        d.child(toggled)
                                })
                        })])
                    })))
                }))
                .apply(|d| d.class("drawer-demo-card").style("height", "350px"))
            }),
            card!({
                .child(content_block!({
                    .title_section(Some(title!({
                        .header_text("Toggled non-modal".to_string())
                    })))
                    .media_section(Some(list!({
                            .rows([
                                html!("div", {
                                    .class("navigation-drawer-demo")
                                    .apply(|d| {
                                        let toggled = toggled(false);
                                        d.child(toggled)
                                    })
                                })
                            ])
                        })))
                }))
                .apply(|d| d.class("drawer-demo-card").style("height", "350px"))
            })
        ])})
    ])
    })
}

fn toggled(modal: bool) -> Dom {
    let expanded = Mutable::new(true);

    navigation_drawer!({
        .expanded_signal(expanded.signal_cloned())
        .with_scrim(true)
        .drawer_content(Some(html!("div", {
            .children(&mut[mock_view_select(), toggle_button(&expanded, "Close")])
        })))
        .modal(modal)
        .main_content(Some(html!("div", {
             .children(&mut[
                html!("div", {
                    .text(lipsum(100).as_str())
                }),
                toggle_button(&expanded, "Show")
            ])
        })))
    })
}

fn retracting(modal: bool) -> Dom {
    let is_extended = Mutable::new(false);

    navigation_drawer!({
        .expanded(true)
        .extended_signal(is_extended.signal())
        .on_extended_change(clone!(is_extended => move |v| {
            is_extended.set_neq(v);
        }))
        .retracts(true)
        .modal(modal)
        .drawer_content(Some(mock_view_select()))
        .main_content(Some(html!("div", {
            .text(lipsum(100).as_str())
        })))
    })
}

pub fn static_drawers(retracts: bool, width: DrawerWidth) -> Dom {
    navigation_drawer!({
        .width(width)
        .retracts(retracts)
        .drawer_content(Some(mock_view_select()))
        .main_content(Some(html!("div", {
            .text(lipsum(100).as_str())
        })))
    })
}

pub fn mock_view_select() -> Dom {
    let entries = MutableVec::new_with_values(vec!["Inbox", "Spam"]);
    let selected_item = Mutable::<Vec<usize>>::new(vec![]);

    let items = entries.signal_vec().map(|entry| ListEntry {
        before: None,
        content: html!("div", { .text(entry.to_string().as_str())}),
        after: None,
    });

    html!("div", {
        .child(interactive_list!({
            .items_signal_vec(items)
            .on_item_selected(move |index| {
                selected_item.set(vec![index]);
            })
        }))
    })
}
