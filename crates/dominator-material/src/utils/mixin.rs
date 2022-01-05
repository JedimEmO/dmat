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
