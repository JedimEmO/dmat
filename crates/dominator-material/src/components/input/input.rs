use dominator::{html, Dom, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::{Broadcaster, Mutable};
use web_sys::HtmlElement;

use crate::components::input::input_props::InputProps;
use crate::components::input::label::label_element;

pub(crate) fn input<F>(
    input_element: Dom,
    has_focus: &Mutable<bool>,
    props: InputProps,
    mixin: F,
    class_name: &str,
    extra_child: Option<Dom>,
) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let label_element = label_element(input_element, &props.value, &has_focus, props.label);

    let mut children = vec![label_element];

    if let Some(extra_child) = extra_child {
        children.push(extra_child);
    }

    let has_assistive = Mutable::new(false);
    let has_error = Mutable::new(false);
    let is_valid = props.is_valid;

    let is_valid = if let Some(is_valid) = is_valid {
        Some(Broadcaster::new(is_valid))
    } else {
        None
    };

    if let Some(error) = props.error_text_signal {
        let has_error = has_error.clone();

        if let Some(valid_sig) = &is_valid {
            let error_text_signal = map_ref!(
                let valid = valid_sig.signal_cloned(),
                let error_text = error => move {
                    if let Some(str) = error_text {
                        if !*valid {
                            has_error.set(true);
                            return Some(crate::text!(str, |d| d.class("dmat-assistive-text").class("dmat-error-text")));
                        }
                    }

                    has_error.set(false);

                    None
                }
            );

            children.push(html!("span", {
                .child_signal(error_text_signal)
            }));
        }
    }

    if let Some(assistive) = props.assistive_text_signal {
        let has_assistive = has_assistive.clone();
        let assistive_element_signal = map_ref!(
            let assistive_text = assistive => move {
                let ass = has_assistive.clone();

                if let Some(str) = assistive_text {
                    ass.set(true);
                    return Some(crate::text!(str, |d| d.class("dmat-assistive-text")))
                }

                ass.set(false);
                None
            }
        );

        children.push(html!("span", {
            .child_signal(assistive_element_signal)
        }));
    }

    html!("div", {
        .children(children.as_mut_slice())
        .apply(mixin)
        .class_signal(
            "assistive",
            map_ref!(
                let assistive = has_assistive.signal(),
                let err = has_error.signal() => {
                    *assistive || *err
                }
            )
        )
        .apply_if(is_valid.is_some(),  move |builder| {
            if let Some(is_valid) = is_valid {
                builder.class_signal("-invalid", map_ref!(let valid = is_valid.signal_cloned() => !valid))
            } else {
                builder
            }
        })
        .class(class_name)
    })
}
