use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::signal::{always, Mutable};
use lipsum::lipsum;
use web_sys::HtmlElement;

use dominator_material::components::{
    ButtonContent, ButtonProps, CardProps, DrawerWidth, NavigationDrawerProps,
};
use dominator_material::utils::mixin::with_stream_flipflop;

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

    let drawer_button_props = ButtonProps {
        content: Some(ButtonContent::Label("Close drawer".to_string())),
        click_handler: clone!(expanded => move |_| {
            expanded.set(!expanded.get())
        }),
        button_type: Default::default(),
        style: Default::default(),
        disabled_signal: always(false),
    };
    let main_button_props = ButtonProps {
        content: Some(ButtonContent::Label("Show drawer".to_string())),
        click_handler: clone!(expanded => move |_| {
            expanded.set(!expanded.get())
        }),
        button_type: Default::default(),
        style: Default::default(),
        disabled_signal: always(false),
    };

    let props = NavigationDrawerProps {
        visible_signal: expanded.signal_cloned(),
        with_scrim: true,
        width: DrawerWidth::Full,
        retracts: false,
        modal,
        drawer_content: html!("div", {
            .child(button!(drawer_button_props))
        }),
        main_content: html!("div", {
             .children(&mut[
                button!(main_button_props),
                html!("div", {
                    .text(lipsum(100).as_str())
                })
            ])
        }),
    };

    let drawer = navigation_drawer!(props);
    let expanded_toggle_mixin =
        with_stream_flipflop(drawer.1.scrim_click_stream.unwrap(), expanded.clone());

    (drawer.0, expanded_toggle_mixin)
}

fn retracting(modal: bool) -> Dom {
    let props = NavigationDrawerProps {
        visible_signal: always(true),
        with_scrim: false,
        width: DrawerWidth::Full,
        retracts: true,
        modal,
        drawer_content: html!("div", {.text("drawer")}),
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
        drawer_content: html!("div", {.text("drawer")}),
        main_content: html!("div", {.text(lipsum(100).as_str())}),
    };

    navigation_drawer!(props).0
}
