use crate::components::scrim::*;
use crate::futures_signals::signal::SignalExt;
use dominator::{clone, events, html, Dom};
use std::sync::Arc;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DrawerWidth {
    Full,
    Narrow,
}

/// Navigation drawer: <https://material.io/components/navigation-drawer>
///
/// # Examples
///
/// ```no_run
/// use dominator::{Dom, html};
/// use dmat_components::components::*;
///
/// fn retracting(modal: bool) -> Dom {
///   navigation_drawer!({
///       .expanded_signal(expanded.signal_cloned())
///       .with_scrim(true)
///       .drawer_content(Some(html!("div", {
///           .children(&mut[mock_view_select(), toggle_button(&expanded, "Close")])
///       })))
///       .retracts(true)
///       .modal(modal)
///       .main_content(Some(html!("div", {
///            .children(&mut[
///               html!("div", {
///                   .text(lipsum(100).as_str())
///               }),
///               toggle_button(&expanded, "Show")
///           ])
///       })))
///   })
/// }
/// ```
#[component(render_fn = navigation_drawer)]
pub struct NavigationDrawer<TOnExtendedChange: Fn(bool) = fn(bool) -> ()> {
    #[signal]
    #[default(true)]
    pub expanded: bool,
    #[signal]
    #[default(true)]
    pub extended: bool,
    pub on_extended_change: TOnExtendedChange,
    /// If true, a scrim will be rendered on top of the contained UI when the drawer is expanded
    #[default(false)]
    pub with_scrim: bool,
    #[signal]
    #[default(DrawerWidth::Full)]
    pub width: DrawerWidth,
    /// Determines if the drawer will collapse and extend based on mouse hover
    #[default(false)]
    pub retracts: bool,
    /// Determines if the drawer overlays the held UI, or if it is render side by side with it
    #[signal]
    #[default(false)]
    pub modal: bool,
    /// The content of the navigation drawer
    #[signal]
    #[default(None)]
    pub drawer_content: Option<Dom>,
    /// The main view which the drawer is attached to
    #[signal]
    #[default(None)]
    pub main_content: Option<Dom>,
}

pub fn navigation_drawer(props: impl NavigationDrawerPropsTrait + 'static) -> Dom {
    let NavigationDrawerProps {
        expanded,
        extended,
        on_extended_change,
        with_scrim,
        width,
        retracts,
        modal,
        drawer_content,
        main_content,
        apply,
    } = props.take();

    let main_content = scrim(
        ScrimProps::new()
            .content_signal(main_content)
            .hide(!with_scrim),
    );

    let extend_cb = on_extended_change.map(Arc::new);
    let width_bc = width.broadcast();

    html!("div", {
        .class("dmat-navigation-drawer")
        .apply_if(apply.is_some(), |d| d.apply(apply.unwrap()))
        .class_signal("-expanded", expanded)
        .class_signal("-extended", extended)
        .class_signal("-modal", modal)
        .class_signal("-narrow", width_bc.signal_ref(|w| *w == DrawerWidth::Narrow))
        .class_signal("-full", width_bc.signal_ref(|w| *w == DrawerWidth::Full))
        .apply_if(retracts, |d| d.class("-retracting"))
        .children(&mut [
            html!("div", {
                .class("drawer")
                .apply_if(retracts, clone!(extend_cb => move |d| {
                    d.event(clone!(extend_cb => move |_:events::MouseEnter| {
                        if let Some(cb) = extend_cb.as_ref() {
                            cb(true);
                        }
                    }))
                    .event(clone!(extend_cb => move |_:events::MouseMove| {
                        if let Some(cb) = extend_cb.as_ref() {
                            cb(true);
                        }
                    }))
                    .event(clone!(extend_cb=> move |_:events::MouseLeave| {
                        if let Some(cb) = extend_cb.as_ref() {
                            cb(false);
                        }
                    }))
                }))

                .child_signal(drawer_content)
            }),
            main_content
        ])
    })
}
