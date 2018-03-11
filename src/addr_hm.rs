//! Hash map implementation that allows keys to be referenced by their hash
//! Instead of looking up type `K`, it can be identified by `hash(K)`.
//! This allows entries in a table to be easily referenced in a `Copy`able way
//!
//! When an element is added to the map, it returns an address that can be used
//!  to handle that entry. That address is a tuple of indices.
//! This is implemented with separate chaining: an address indices are
//!  (1) the index in the main table and (2) the index in the bucket
//! Searching within a bucket is practically fast (linear search on few elems),
//!  but imposing a constant upper limit on the size can fix the order notation
//!
//! This creates some funkiness. Elements cannot be erased and table resizing is
//!  awkward (it returns a mapping of how `Addr`s change).
//!  Both of these can be addressed higher up. Resizing can be accomplished 
//!   with a scan-and-replace of `Addr`s and `remove`ing can ignore an `Addr`,
//!   it just can't actually remove it from memory.
//!  Even so. Cool stuff.

#![allow(unused)]



fn main() {}

use std::fmt;
use std::ops::{Index, IndexMut};
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::marker::PhantomData;

const RESIZE_THRESHOLD: f64 = 1.5;
const RESIZE_FACTOR: f64 = 1.5;
const MAX_BUCKET_SIZE: usize = 32;
const DEFAULT_TABLE_CAPACITY: usize = 32;
const DEFAULT_BUCKET_CAPACITY: usize = 4;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Addr {
    table: usize,
    bucket: usize,
    #[cfg(debug_assertions)] sig: Signature,
}

#[cfg(debug_assertions)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Signature(usize);

#[cfg(debug_assertions)]
impl Signature {
    fn random() -> Self {
        // TODO
        Signature(4) //chosen by fair dice roll
    }
}


#[derive(Debug)]
pub(crate) struct AddrHashMap<T: fmt::Debug+Hash+Eq, H: Hasher + Default> {
    capacity: usize,
    size: usize,
    table: Box<[Vec<T>]>,
    _hasher: PhantomData<H>,
    #[cfg(debug_assertions)] sig: Signature,
}

impl<T: fmt::Debug+Hash+Eq> Default for AddrHashMap<T, DefaultHasher> {
    fn default() -> Self {
        AddrHashMap::with_capacity(DEFAULT_TABLE_CAPACITY)
    }
}

impl<T: fmt::Debug+Hash+Eq> AddrHashMap<T, DefaultHasher> {
    pub(crate) fn with_capacity(c: usize) -> Self {
        Self::with_capacity_and_hasher(c, DefaultHasher::new())
    }

}

impl<T: fmt::Debug+Hash+Eq, H: Hasher+Default> AddrHashMap<T, H> {
    /// Create new `AddrHashMap` with specified capacity and hasher
    pub(crate) fn with_capacity_and_hasher(c: usize, h: H) -> Self {
        // better way?
        let v: Vec<Vec<T>> = (0..c).map(|_| vec![]).collect();
        AddrHashMap {
            size: 0,
            capacity: DEFAULT_TABLE_CAPACITY,
            table: v.into_boxed_slice(),
            _hasher: PhantomData,
            #[cfg(debug_assertions)] sig: Signature::random(),
        }
    }
    /// Create a new, larger `AddrHashMap` and copy the data over
    /// Return a translation map of old `Addr`s to new ones
    // TODO do not copy over "deleted" elements?
    pub(crate) fn from_old(old: Self) -> (Self, HashMap<Addr,Addr>) {
        unimplemented!()
    }

    fn hash(val: &T) -> usize {
        let mut hasher = H::default();
        val.hash(&mut hasher);
        hasher.finish() as usize
    }

    /// Insert an element, and get its referencable address
    /// If the table is "full", then return a `None`
    pub(crate) fn insert(&mut self, val: T) -> Option<Addr> {
        if self.size as f64 / self.capacity as f64 > RESIZE_THRESHOLD {
            return None;
        }
        let hash = Self::hash(&val);
        let table_index = hash % self.capacity;
        let bucket = &mut self.table[table_index];
        let bucket_index = bucket.len();
        if bucket.len() >= MAX_BUCKET_SIZE {
            return None;
        }
        bucket.push(val);
        Some(Addr {
            table: table_index,
            bucket: bucket_index,
            #[cfg(debug_assertions)] sig: self.sig,
        })
    }

    /// Locates an element by its reference or returns `None` if it's absent
    pub(crate) fn get(&self, val: &T) -> Option<Addr> {
        let hash = Self::hash(&val);
        let table_index = hash % self.capacity;
        let bucket = &self.table[table_index];
        for (index,elem) in bucket.iter().enumerate() {
            if elem == val {
                return Some(Addr {
                    table: table_index,
                    bucket: index,
                    #[cfg(debug_assertions)] sig: self.sig,
                });
            }
        }
        None
    }
}

impl<T: fmt::Debug+Hash+Eq, H: Hasher+Default> Index<Addr> for AddrHashMap<T, H> {
    type Output = T;
    fn index(&self, addr: Addr) -> &T {
        debug_assert_eq!(self.sig, addr.sig);
        let bucket = &self.table[addr.table];
        &bucket[addr.bucket]
    }
}

impl<T: fmt::Debug+Hash+Eq, H: Hasher+Default> IndexMut<Addr> for AddrHashMap<T, H> {
    fn index_mut(&mut self, addr: Addr) -> &mut T {
        debug_assert_eq!(self.sig, addr.sig);
        let bucket = &mut self.table[addr.table];
        &mut bucket[addr.bucket]
    }
}



