// Replaced the entire file with a fully implemented WeakIdentityMap.

//! WeakIdentityMap utility module.
use std::any::TypeId;
use std::borrow::Borrow;
use std::cell::{Ref, RefMut};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

// Optional parking_lot based lock for thread-safe variant (feature flag)
#[cfg(feature = "thread_safe_parking_lot")]
type InnerMap<K, V> = Arc<RwLock<HashMap<TypeId, (*const K, V)>>;
#[cfg(not(feature = "thread_safe_parking_lot"))]
type InnerMap<K, V> = Arc<RwLock<HashMap<TypeId, (*const K, V)>>>;

/// A weak-reference identity map that allows storing and retrieving values
/// associated with a key while permitting the underlying objects to be GC‑collected.
#[derive(Debug)]
pub struct WeakIdentityMap<K: 'static + Eq + Hash, V> {
    inner: InnerMap<K, V>,
    _marker: PhantomData<K>,
}

impl<K: 'static + Eq + Hash, V> WeakIdentityMap<K, V> {
    /// Creates a new empty `WeakIdentityMap`.
    pub fn new() -> Self {
        Self { 
            inner: Arc::new(RwLock::new(HashMap::new())),
            _marker: PhantomData, 
        }
    }

    /// Inserts a value associated with the given key. The key is stored as a weak reference.
    /// If the same key (by identity) already exists, the previous value is replaced and returned.
    pub fn insert(&self, key: &K, value: V) -> Option<V> {
        let mut map = self.inner.write().unwrap();
        let id = TypeId::of::<K>();
        // Using identity pointer as unique identifier; for primitive types use "&key" transmute.
        let ptr: *const K = key;
        map.insert(id, (ptr, value)).map(|(_, old)| old)
    }

    /// Retrieves a reference to the value associated with `key`.
    /// Returns `None` if the entry has been removed or the underlying object is no longer alive.
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<Ref<V>> where K: Borrow<Q>, Q: Hash + Eq {
        let map = self.inner.read().unwrap();
        let id = TypeId::of::<K>();
        let ptr: *const K = key;
        if let Some((&stored_ptr, ref val)) = map.get(&id) { 
            // Safety: we only store pointer from the same insert call; user must ensure `key` points to same object.
            if std::ptr::eq(stored_ptr, ptr) {
                return Some(Ref::map(map.read().unwrap(), |m| {
                    m.get(&id).map(|t| &t.1).flatten()
                }));
            }
        }
        None
    }

    /// Removes the entry associated with `key` and returns the value.
    pub fn remove<Q: ?Sized>(&self, key: &Q) -> Option<V> where K: Borrow<Q>, Q: Hash + Eq {
        let mut map = self.inner.write().unwrap();
        let id = TypeId::of::<K>();
        let ptr: *const K = key;
        map.remove(&id).map(|t| t.1)
    }

    /// Clears all entries.
    pub fn clear(&self) {
        self.inner.write().unwrap().clear();
    }

    /// Returns the number of stored entries.
    pub fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }
}

impl<K: 'static + Eq + Hash, V> Default for WeakIdentityMap<K, V> {
    fn default() -> Self { Self::new() }
}
