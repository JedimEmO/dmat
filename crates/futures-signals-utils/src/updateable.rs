use futures_signals::signal::Mutable;
use futures_signals::signal_map::MutableBTreeMap;
use futures_signals::signal_vec::MutableVec;
use std::cmp::min;
use std::hash::Hash;
use std::ops::Deref;

pub trait Updateable {
    fn update(&self, other: Self);
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
    fn update(&self, other: Self) {
        self.1.update(other.1)
    }
}

impl<T> Updateable for MutableVec<T>
where
    T: Updateable + Clone,
{
    fn update(&self, other: Self) {
        let mut self_lock = self.lock_mut();
        let mut other_lock = other.lock_mut();

        let self_len = self_lock.len();
        let other_len = other_lock.len();

        let last_common_idx = min(self_len, other_len);

        let other_moved = other_lock.drain(..last_common_idx);

        self_lock
            .iter()
            .zip(other_moved)
            .take(last_common_idx)
            .for_each(|(s, o)| s.update(o));

        match self_len.cmp(&other_len) {
            std::cmp::Ordering::Less => {
                self_lock.extend(other_lock.drain(..));
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
    fn update(&self, other: Self) {
        {
            let s = self.lock_ref();
            let o = other.lock_ref();

            if s.deref() == o.deref() {
                return;
            }
        }

        self.swap(&other);
    }
}

impl<T, K> Updateable for MutableBTreeMap<K, T>
where
    T: Updateable + Clone,
    K: PartialEq + Ord + Clone,
{
    fn update(&self, other: Self) {
        let keys_to_remove = self
            .lock_ref()
            .iter()
            .filter(|(k, _)| !other.lock_ref().contains_key(k))
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>();

        let mut self_lock = self.lock_mut();
        for key in keys_to_remove {
            self_lock.remove(&key);
        }

        for (key, value) in other
            .lock_mut()
            .iter()
            .map(|(k, v)| (k.clone(), (*v).clone()))
        {
            match self_lock.get(&key) {
                Some(existing) => existing.update(value),
                None => {
                    self_lock.insert_cloned(key.clone(), value);
                }
            }
        }
    }
}

pub fn update_vec_direct_cloned<T: PartialEq + Clone>(
    target: &MutableVec<T>,
    other: MutableVec<T>,
) {
    let mut target_lock = target.lock_mut();
    let mut other_lock = other.lock_mut();

    let self_len = target_lock.len();
    let other_len = other_lock.len();

    let last_common_idx = min(self_len, other_len);

    {
        let mut other_moved = other_lock.drain(..last_common_idx);

        for i in 0..last_common_idx {
            let next_other = other_moved.next().unwrap();

            if target_lock[i] == next_other {
                continue;
            }

            target_lock.set_cloned(i, next_other);
        }
    }

    match self_len.cmp(&other_len) {
        std::cmp::Ordering::Less => {
            target_lock.extend(other_lock.drain(..));
        }
        std::cmp::Ordering::Greater => {
            target_lock.truncate(other_len);
        }
        std::cmp::Ordering::Equal => {}
    }
}

pub fn update_vec_direct_copied<T: PartialEq + Copy>(target: &MutableVec<T>, other: MutableVec<T>) {
    let mut target_lock = target.lock_mut();
    let mut other_lock = other.lock_mut();

    let self_len = target_lock.len();
    let other_len = other_lock.len();

    let last_common_idx = min(self_len, other_len);

    {
        let mut other_moved = other_lock.drain(..last_common_idx);

        for i in 0..last_common_idx {
            let next_other = other_moved.next().unwrap();

            if target_lock[i] == next_other {
                continue;
            }

            target_lock.set(i, next_other);
        }
    }

    match self_len.cmp(&other_len) {
        std::cmp::Ordering::Less => {
            target_lock.extend(other_lock.drain(..));
        }
        std::cmp::Ordering::Greater => {
            target_lock.truncate(other_len);
        }
        std::cmp::Ordering::Equal => {}
    }
}
