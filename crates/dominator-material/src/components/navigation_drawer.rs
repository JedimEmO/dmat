use dominator::{clone, events, html, Dom, DomBuilder};
use futures::channel::mpsc::Receiver;
use futures_signals::signal::always;
use futures_signals::signal::{Mutable, MutableSignalCloned, Signal};
use web_sys::HtmlElement;

use crate::components::ScrimProps;
use crate::scrim;

#[derive(Copy, Clone, PartialEq)]
pub enum DrawerWidth {
    Full,
    Narrow,
}

pub struct NavigationDrawerProps<TVisibleSignal: Signal<Item = bool>> {
    pub visible_signal: TVisibleSignal,
    /// If true, a scrim will be rendered on top of the contained UI when the drawer is expanded
    pub with_scrim: bool,
    pub width: DrawerWidth,
    /// Determines if the drawer will collapse and extend based on mouse hover
    pub retracts: bool,
    /// Determines if the drawer overlays the held UI, or if it is render side by side with it
    pub modal: bool,
    /// The content of the navigation drawer
    pub drawer_content: Dom,
    /// The main view which the drawer is attached to
    pub main_content: Dom,
}

pub struct NavigationDrawerOut {
    /// If the drawer is configured to retract, this signal will contain the extended state
    pub is_extended: Option<MutableSignalCloned<bool>>,
    pub scrim_click_stream: Option<Receiver<()>>,
}

#[macro_export]
macro_rules! navigation_drawer {
    ($props: expr) => {{
        $crate::components::navigation_drawer::navigation_drawer($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::navigation_drawer::nacarvigation_drawer($props, $mixin)
    }};
}

/// Navigation drawer: <https://material.io/components/navigation-drawer>
///
/// # Examples
///
/// ```no_run
/// use dominator::{Dom, html};
/// use futures_signals::signal::always;
/// use dominator_material::components::{NavigationDrawerProps, DrawerWidth};
///
/// fn retracting(modal: bool) -> Dom {
///     let props = NavigationDrawerProps {
///         visible_signal: always(true),
///         with_scrim: false,
///         width: DrawerWidth::Full,
///         retracts: true,
///         modal,
///         drawer_content: html!("div", {.text("This is the content of the drawer. Put menu items etc. here!")}),
///         main_content: html!("div", {.text("This is the main view, a modal drawer will cover it, whereas a non-modal drawer will displace it")}),
///     };
///
///     dominator_material::navigation_drawer!(props).0
/// }
/// ```
pub fn navigation_drawer<F, TVisibleSignal>(
    props: NavigationDrawerProps<TVisibleSignal>,
    mixin: F,
) -> (Dom, NavigationDrawerOut)
where
    TVisibleSignal: Signal<Item = bool> + 'static,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let expanded = props.visible_signal;
    let width = props.width;
    let extended = Mutable::new(false);
    let retracts = props.retracts;
    let main_content = props.main_content;
    let drawer_content = props.drawer_content;

    let (main_content, scrim_click_stream) = match props.with_scrim {
        true => {
            let (main_out, scrim_out) = scrim!(ScrimProps {
                content: main_content,
                hide_signal: always(false)
            });

            (main_out, Some(scrim_out.click_stream))
        }
        _ => (main_content, None),
    };

    let out = NavigationDrawerOut {
        is_extended: match retracts {
            true => Some(extended.signal_cloned()),
            _ => None,
        },
        scrim_click_stream,
    };

    (
        html!("div", {
            .class("dmat-navigation-drawer")
            .apply(mixin)
            .class_signal("-expanded", expanded)
            .class_signal("-extended", extended.signal_cloned())
            .apply_if(props.retracts, |d| d.class("-retracting"))
            .apply_if(props.modal, |d| d.class("-modal"))
            .class(drawer_width_to_css_class(width))
            .children(&mut [
                html!("div", {
                    .class("drawer")
                    .apply_if(retracts, clone!(extended => move |d| {
                        d.event(clone!(extended => move |_:events::MouseEnter| {
                            extended.set(true);
                        }))
                        .event(clone!(extended=> move |_:events::MouseLeave| {
                            extended.set(false);
                        }))
                    }))

                    .child(drawer_content)
                }),
                main_content
            ])
        }),
        out,
    )
}

fn drawer_width_to_css_class(width: DrawerWidth) -> &'static str {
    match width {
        DrawerWidth::Narrow => "-narrow",
        DrawerWidth::Full => "-full",
    }
}
