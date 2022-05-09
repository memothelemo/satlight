use std::{
    fmt::Debug,
    hash::Hash,
    ops::RangeBounds,
    vec::{Drain, IntoIter},
};

/// It almost functions the same as the other programming languages.
///
/// Think of `HashMap` but without hash stuff involved.
#[derive(Default, Clone)]
pub struct Dictionary<K, V> {
    entries: Vec<(K, V)>,
}

impl<K, V> std::fmt::Debug for Dictionary<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.entries.fmt(f)
    }
}

impl<K, V> Hash for Dictionary<K, V>
where
    K: Hash,
    V: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for entry in self.entries.iter() {
            entry.0.hash(state);
            entry.1.hash(state);
        }
    }
}

impl<K, V> Dictionary<K, V>
where
    K: PartialEq,
    V: PartialEq,
{
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        self.entries.capacity()
    }

    pub fn contains(&self, key: &K) -> bool {
        for (entry, _) in self.entries.iter() {
            if entry == key {
                return true;
            }
        }
        false
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.id_from_key(key)
            .and_then(|v| self.entries.get(v).as_ref().map(|v| &v.1))
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let id = self.id_from_key(key);
        if let Some(v) = id {
            let entry = self.entries.get_mut(v);
            if let Some(entry) = entry {
                return Some(&mut entry.1);
            }
        }
        None
    }

    pub fn get_retrieve_id(&self, key: &K) -> Option<(usize, &V)> {
        self.id_from_key(key)
            .and_then(|id| self.entries.get(id).as_ref().map(|v| (id, &v.1)))
    }

    pub fn id_from_key(&self, key: &K) -> Option<usize> {
        for (id, (entry, _)) in self.entries.iter().enumerate() {
            if entry == key {
                return Some(id);
            }
        }
        None
    }

    pub fn insert(&mut self, key: K, value: V) {
        // optimizations: we can replace the entry if it does exists
        match self.id_from_key(&key) {
            Some(id) => self.entries.get_mut(id).unwrap().1 = value,
            None => self.entries.push((key, value)),
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.id_from_key(key) {
            Some(id) => Some(self.entries.remove(id).1),
            None => None,
        }
    }

    pub fn append(&mut self, other: &mut Dictionary<K, V>) {
        self.entries.append(&mut other.entries);
    }

    pub fn drain<R>(&mut self, range: R) -> Drain<'_, (K, V)>
    where
        R: RangeBounds<usize>,
    {
        self.entries.drain(range)
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn truncate(&mut self, len: usize) {
        self.entries.truncate(len);
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn pick_limit(&self, limit: usize) -> Vec<&(K, V)> {
        let mut count = 0;
        let mut list = Vec::new();
        while count < limit {
            let member = self.entries.get(count);
            if let Some(member) = member {
                list.push(member);
                count += 1;
            } else {
                break;
            }
        }
        list
    }

    pub fn filter(&self, filter_call: impl Fn(&K, &V) -> bool) -> Vec<&(K, V)> {
        let mut list = Vec::new();
        for entry in self.entries.iter() {
            if filter_call(&entry.0, &entry.1) {
                list.push(entry);
            }
        }
        list
    }
}

impl<K, V> std::ops::Deref for Dictionary<K, V> {
    type Target = [(K, V)];

    fn deref(&self) -> &Self::Target {
        self.entries.deref()
    }
}

impl<K, V> std::ops::DerefMut for Dictionary<K, V> {
    fn deref_mut(&mut self) -> &mut [(K, V)] {
        self.entries.deref_mut()
    }
}

impl<K, V> IntoIterator for Dictionary<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}
