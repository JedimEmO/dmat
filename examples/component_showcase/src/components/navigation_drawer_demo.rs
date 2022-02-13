use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::signal::{always, Mutable};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use lipsum::lipsum;
use web_sys::HtmlElement;

use dominator_material::components::{
    CardProps, DrawerWidth, InteractiveListProps, ListEntry, NavigationDrawerProps,
};
use dominator_material::utils::mixin::with_stream_flipflop;
use dominator_material::utils::mixin::with_stream_value;

use crate::utils::toggle_button::toggle_button;

#[inline]
pub fn navigation_drawers_demo() -> Dom {
    static_list!(vec![
        container!(|d| d.children(&mut [
            card!(
                CardProps::new()
                    .with_title("Retracting modal drawer", None)
                    .body(html!("div", {
                        .class("navigation-drawer-demo")
                        .child(retracting(true))
                    })),
                |d| d.class("drawer-demo-card").style("height", "350px")
            ),
            card!(
                CardProps::new()
                    .with_title("Retracting non-modal drawer", None)
                    .body(html!("div", {
                        .class("navigation-drawer-demo")
                        .child(retracting(false))
                    })),
                |d| d.class("drawer-demo-card").style("height", "350px")
            ),
        ])),
        container!(|d| d.children(&mut [
            card!(
                CardProps::new()
                    .with_title("Modal toggled", None)
                    .body(html!("div", {
                        .class("navigation-drawer-demo")
                        .apply(|d| {
                            let (toggled, mixin) = toggled(true);
                            d.child(toggled)
                            .apply(mixin)
                        })                    })),
                |d| d.class("drawer-demo-card").style("height", "350px")
            ),
            card!(
                CardProps::new()
                    .with_title("Toggled non-modal", None)
                    .body(html!("div", {
                        .class("navigation-drawer-demo")
                        .apply(|d| {
                            let (toggled, mixin) = toggled(false);
                            d.child(toggled)
                            .apply(mixin)
                        })
                    })),
                |d| d.class("drawer-demo-card").style("height", "350px")
            )
        ]))
    ])
}

fn toggled(
    modal: bool,
) -> (
    Dom,
    impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
) {
    let expanded = Mutable::new(true);

    let props = NavigationDrawerProps {
        visible_signal: expanded.signal_cloned(),
        with_scrim: true,
        width: DrawerWidth::Full,
        retracts: false,
        modal,
        drawer_content: html!("div", {
            .children(&mut[mock_view_select(), toggle_button(&expanded, "Close")])
        }),
        main_content: html!("div", {
             .children(&mut[
                html!("div", {
                    .text(lipsum(100).as_str())
                }),
                toggle_button(&expanded, "Show")
            ])
        }),
    };

    let drawer = navigation_drawer!(props);
    let expanded_toggle_mixin =
        with_stream_flipflop(drawer.1.scrim_click_stream.unwrap(), &expanded);

    (drawer.0, expanded_toggle_mixin)
}

fn retracting(modal: bool) -> Dom {
    let props = NavigationDrawerProps {
        visible_signal: always(true),
        with_scrim: false,
        width: DrawerWidth::Full,
        retracts: true,
        modal,
        drawer_content: mock_view_select(),
        main_content: html!("div", {.text(lipsum(100).as_str())}),
    };

    navigation_drawer!(props).0
}

pub fn static_drawers(retracts: bool, width: DrawerWidth) -> Dom {
    let props = NavigationDrawerProps {
        visible_signal: always(true),
        with_scrim: false,
        width,
        retracts,
        modal: false,
        drawer_content: mock_view_select(),
        main_content: html!("div", {.text(lipsum(100).as_str())}),
    };

    navigation_drawer!(props).0
}

pub fn mock_view_select() -> Dom {
    let entries = MutableVec::new_with_values(vec!["Inbox", "Spam"]);
    let selected_item = Mutable::<Option<&str>>::new(None);

    let items = entries
        .signal_vec()
        .map(clone!(selected_item => move |entry| ListEntry {
            before: None,
            content: html!("div", { .text(format!("{}", entry).as_str())}),
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
        .apply(with_stream_value(out.item_select_stream, &selected_item))
    })
}
