use dominator::Dom;
use futures_signals::signal::{always, Always};
use std::iter::{once, Once};

pub fn once_cmp(c: Dom) -> Once<Always<Option<Dom>>> {
    once(always(Some(c)))
}
