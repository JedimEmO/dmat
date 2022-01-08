use dominator::DomBuilder;
use web_sys::Element;

#[inline]
pub fn no_mixin<A: AsRef<Element>>(_dom_builder: DomBuilder<A>) -> DomBuilder<A> {
    _dom_builder
}

#[inline]
pub fn mixin_id<A: AsRef<Element>>() -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A> {
    no_mixin::<A>
}

#[inline]
pub fn with_id<T: AsRef<str> + 'static, A: AsRef<Element>>(
    id: T,
) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A> {
    move |d: DomBuilder<A>| d.attribute("id", id.as_ref())
}
