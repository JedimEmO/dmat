use dominator::{clone, html, Dom};
use futures_signals::signal::from_stream;
use futures_signals::signal::{always, Mutable};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use lipsum::lipsum;

use dmat_components::components::layouts::*;
use dmat_components::components::*;
use dmat_components::utils::signals::mutation::store_signal_value_opt_mixin;

use crate::utils::toggle_button::toggle_button;

#[inline]
pub fn navigation_drawers_demo() -> Dom {
    list!({
        .rows([
        container!({.children([
            card!({
                .child(content_block!(ContentBlockProps {
                    title_section: Some(title!(TitleProps {
                        header_text_signal: always("Retracting modal drawer".to_string()),
                        sub_header_text_signal: always(None)
                    })),
                    media_section: Some(list!({
                            .rows([html!("div", {
                            .class("navigation-drawer-demo")
                            .child(retracting(true))
                        })])
                    })),
                    supporting_section: None,
                    footer_section: None
                }))
                .apply(|d| d.class("drawer-demo-card").style("height", "350px"))
            }),
            card!({
                .child( content_block!(ContentBlockProps {
                    title_section: Some(title!(TitleProps {
                        header_text_signal: always("Retracting non-modal drawer".to_string()),
                        sub_header_text_signal: always(None)
                    })),
                    media_section: Some(list!({
                        .rows([
                                html!("div", {
                                .class("navigation-drawer-demo")
                                .child(retracting(false))
                            })
                        ])
                    })),
                    supporting_section: None,
                    footer_section: None
                }))
                .apply(|d| d.class("drawer-demo-card").style("height", "350px"))
            }),
        ])}),
        container!({.children([
            card!({
                .child(content_block!(ContentBlockProps {
                    title_section: Some(title!(TitleProps {
                        header_text_signal: always("Modal toggled".to_string()),
                        sub_header_text_signal: always(None)
                    })),
                    media_section: Some(list!({
                            .rows([
                                html!("div", {
                                    .class("navigation-drawer-demo")
                                    .apply(|d| {
                                        let toggled = toggled(true);
                                        d.child(toggled)
                                })
                        })])
                    })),
                    supporting_section: None,
                    footer_section: None
                }))
                .apply(|d| d.class("drawer-demo-card").style("height", "350px"))
            }),
            card!({
                .child(content_block!(ContentBlockProps {
                    title_section: Some(title!(TitleProps {
                        header_text_signal: always("Toggled non-modal".to_string()),
                        sub_header_text_signal: always(None)
                    })),
                    media_section: Some(list!({
                            .rows([
                                html!("div", {
                                    .class("navigation-drawer-demo")
                                    .apply(|d| {
                                        let toggled = toggled(false);
                                        d.child(toggled)
                                    })
                                })
                            ])
                        })),
                    supporting_section: None,
                    footer_section: None
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
    let selected_item = Mutable::<Option<&str>>::new(None);

    let items = entries
        .signal_vec()
        .map(clone!(selected_item => move |entry| ListEntry {
            before: None,
            content: html!("div", { .text(entry.to_string().as_str())}),
            after: None,
            selected_signal: Box::new(
                selected_item.signal_ref(clone!(entry => move |v| v == &Some(entry))),
            ),
            item_value: entry
        }));

    let props = InteractiveListProps { items };
    let (list_body, out) = interactive_list!(props);

    html!("div", {
        .child(list_body)
        .apply(store_signal_value_opt_mixin(from_stream(out.item_select_stream), &selected_item))
    })
}
