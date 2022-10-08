use dominator::__internal::SvgElement;
use dominator::{clone, DomBuilder};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::rc::Rc;
use std::time::Duration;

pub fn animated_attribute<T: 'static>(
    builder: DomBuilder<SvgElement>,
    value_sig: Box<dyn Signal<Item = T> + Unpin>,
    attr_function: Rc<dyn Fn(T) -> String>,
    attr_name: String,
    duration: Duration,
) -> DomBuilder<SvgElement> {
    let concrete_attr: Mutable<Option<String>> = Mutable::new(None);
    let current_attr = Mutable::new("".to_string());
    let old_attr = Mutable::new("".to_string());

    builder
        .future(clone!(old_attr, current_attr, concrete_attr, attr_function => async move {
            value_sig.for_each(|data| {
                if current_attr.get_cloned() == "" {
                    let points = attr_function(data);
                    concrete_attr.set(Some(points.clone()));
                    current_attr.set(points);
                } else {
                    concrete_attr.set(None);
                    old_attr.set(current_attr.get_cloned());
                    current_attr.set(attr_function(data));
                }

                async {}
            }).await;
        }))
        .attr_signal(attr_name.clone(), clone!(concrete_attr => {
                map_ref! {
                    let cp = concrete_attr.signal_cloned() => {
                        match cp {
                            Some(v) => v.clone(),
                            None => "".to_string()
                        }
                    }
                }
            }))
        .child_signal(clone!(old_attr, current_attr, concrete_attr, attr_name, duration => {
            map_ref! {
                let current_attr = current_attr.signal_cloned(), let cp = concrete_attr.signal_cloned() => move {
                    if cp.is_some() {
                        return None;
                    }

                    Some(svg!("animate" => web_sys::SvgAnimationElement, {
                        .after_inserted(|e| {
                            e.begin_element().unwrap();
                        })
                        .attr("attributeName", attr_name.as_str())
                        .attr("dur", format!("{:.2}s", duration.as_secs_f32()).as_str())
                        .attr("repeatCount", "1")
                        .attr("fill", "freeze")
                        .attr("to", current_attr.as_str())
                        .attr("from", old_attr.get_cloned().as_str())
                        .future(clone!(concrete_attr, current_attr,duration => async move {
                            wasm_timer::Delay::new(duration).await.unwrap();
                            concrete_attr.set(Some(current_attr))
                        }))
                    }))
                }
            }
        }))
}
