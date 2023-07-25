use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use std::cmp::min;

use std::hash::Hash;
use std::ops::Deref;

pub trait Updateable {
    fn update(&self, other: &Self);
}

pub trait Identifiable {
    type Key: Eq + Hash + Clone;
    fn id(&self) -> &Self::Key;
}

impl<K, V> Identifiable for (K, V)
where
    K: Eq + Hash + Clone,
{
    type Key = K;

    fn id(&self) -> &K {
        &self.0
    }
}

impl<K, V> Updateable for (K, V)
where
    V: Updateable,
{
    fn update(&self, other: &Self) {
        self.1.update(&other.1)
    }
}

impl<T> Updateable for MutableVec<T>
where
    T: Updateable + Clone,
{
    fn update(&self, other: &Self) {
        let mut self_lock = self.lock_mut();
        let mut other_lock = other.lock_mut();

        let self_len = self_lock.len();
        let other_len = other_lock.len();

        let last_common_idx = min(self_len, other_len);

        self_lock
            .iter()
            .zip(other_lock.iter())
            .take(last_common_idx)
            .for_each(|(s, o)| s.update(o));

        match self_len.cmp(&other_len) {
            std::cmp::Ordering::Less => {
                self_lock.extend(other_lock.drain(self_len..));
            }
            std::cmp::Ordering::Greater => {
                self_lock.truncate(other_len);
            }
            std::cmp::Ordering::Equal => {}
        }
    }
}

impl<T> Updateable for Mutable<T>
where
    T: PartialEq,
{
    fn update(&self, other: &Self) {
        {
            let s = self.lock_ref();
            let o = other.lock_ref();

            if s.deref() == o.deref() {
                return;
            }
        }

        self.swap(other);
    }
}
