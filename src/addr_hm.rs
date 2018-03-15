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

//#[cfg(sig, debug_assertions)]

extern crate rand;

use std::fmt::Debug;
use std::borrow::Borrow;
use std::ops::{Index, IndexMut};
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::marker::PhantomData;
use std::vec;

//use vertex::NodeT;

const RESIZE_THRESHOLD: f64 = 1.5;
const RESIZE_FACTOR: f64 = 2.0;
const MAX_BUCKET_SIZE: usize = 32;
const DEFAULT_TABLE_CAPACITY: usize = 32;
const DEFAULT_BUCKET_CAPACITY: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Addr<T: Debug+Eq+Hash> {
    table: usize,
    bucket: usize,
    _type: PhantomData<T>,
    #[cfg(debug_assertions)] sig: Signature,
}

impl<T: Debug+Eq+Hash> Addr<T> {
    #[cfg(debug_assertions)]
    fn from(table: usize, bucket: usize, sig: Signature) -> Self {
        Addr { table, bucket, sig, _type: PhantomData }
    }
    #[cfg(not(debug_assertions))]
    fn from(table: usize, bucket: usize) -> Self {
        Addr { table, bucket, _type: PhantomData }
    }
}


#[cfg(debug_assertions)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Signature(usize);

#[cfg(debug_assertions)]
impl Signature {
    fn random() -> Self {
        Signature(rand::random())
    }
}

pub(crate) type AddrSet<T> = AddrHashSet<T, DefaultHasher>;

#[derive(Debug)]
pub(crate) struct AddrHashSet<T: Debug+Eq+Hash, H: Hasher + Default> {
    capacity: usize,
    size: usize,
    table: Box<[Vec<T>]>,
    _hasher: PhantomData<H>,
    #[cfg(debug_assertions)] sig: Signature,
}

impl<T: Debug+Eq+Hash> Default for AddrHashSet<T, DefaultHasher> {
    fn default() -> Self {
        AddrHashSet::with_capacity(DEFAULT_TABLE_CAPACITY)
    }
}

impl<T: Debug+Eq+Hash> AddrHashSet<T, DefaultHasher> {
    pub(crate) fn with_capacity(c: usize) -> Self {
        Self::with_capacity_and_hasher(c)
    }

}

impl<T: Debug+Eq+Hash, H: Hasher+Default> AddrHashSet<T, H> {
    /// Create new `AddrHashSet` with specified capacity and hasher
    pub(crate) fn with_capacity_and_hasher(c: usize) -> Self {
        // better way?
        let v: Vec<Vec<T>> = (0..c).map(|_| vec![]).collect();
        AddrHashSet {
            size: 0,
            capacity: DEFAULT_TABLE_CAPACITY,
            table: v.into_boxed_slice(),
            _hasher: PhantomData,
            #[cfg(debug_assertions)] sig: Signature::random(),
        }
    }
    /// Create a new, larger `AddrHashSet` and copy the data over
    /// Return a translation map of old `Addr`s to new ones
    // TODO do not copy over "deleted" elements?
    pub(crate) fn from_old(old: Self) -> (Self, HashMap<Addr<T>,Addr<T>>) {
        let new_cap = (old.capacity() as f64 * RESIZE_FACTOR) as usize;
        let mut replacements = HashMap::with_capacity(old.len());
        let mut new = Self::with_capacity_and_hasher(new_cap);

        for (val, old_addr) in old.into_iter_1() {
            // TODO will this ever panic? maybe if a bucket overflows?
            // If new_cap is twice old_cap, every bucket should be no fuller
            // I think
            let new_addr = new.insert(val).unwrap();
            replacements.insert(old_addr, new_addr);
        }

        (new, replacements)
    }

    pub(crate) fn len(&self) -> usize { self.size }
    pub(crate) fn is_empty(&self) -> bool { self.size == 0 }
    pub(crate) fn capacity(&self) -> usize { self.capacity }

    fn hash<S: Hash>(val: &S) -> usize {
        let mut hasher = H::default();
        val.hash(&mut hasher);
        hasher.finish() as usize
    }

    /// Insert an element, and get its referencable address
    /// If the table is "full", then return a `None`
    pub(crate) fn insert(&mut self, val: T) -> Option<Addr<T>> {
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
        //Some(Addr::from(table_index, bucket_index, #[cfg(debug_assertions)] self.sig))
        Some(Addr {
            table: table_index,
            bucket: bucket_index,
            _type: PhantomData,
            #[cfg(debug_assertions)] sig: self.sig,
        })
    }

    /*
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
    */

    /*
    pub(crate) fn contains_key(&self, val: &T) -> bool {
        let table_index = Self::hash(&val) % self.capacity;
        &self.table[table_index].iter().any(|elem| elem == val)
    }
    */

    pub(crate) fn iter<'a>(&'a self) -> Box<Iterator<Item=(&'a T, Addr<T>)>+'a> {
        let iter = self.table.iter().enumerate()
            .flat_map(move |(t_i, bucket)|
                      bucket.iter()
                      .enumerate()
                      .map(move |(b_i, t)| 
                           (t, Addr {
                               table: t_i,
                               bucket: b_i,
                               _type: PhantomData,
                               #[cfg(debug_assertions)] sig: self.sig,
                           }))
                      );
        Box::new(iter)
    }

    pub(crate) fn into_iter_1(self) -> vec::IntoIter<(T,Addr<T>)> {
        // Note: this collects to a Vec first
        #[cfg(debug_assertions)] let sig = self.sig;
        let table: Vec<Vec<T>> = self.table.into();
        let elems: Vec<(T,Addr<T>)> = table.into_iter().enumerate()
            .flat_map(|(t_i, bucket)| 
                      bucket.into_iter()
                      .enumerate()
                      .map(move |(b_i, t)|
                           (t, Addr {
                               table: t_i, 
                               bucket: b_i,
                               _type: PhantomData,
                               #[cfg(debug_assertions)] sig,
                           }))
                      )
            .collect();
        elems.into_iter()
    }

}

impl<T: 'static + Debug+Eq+Hash, H: Hasher+Default> AddrHashSet<T, H> {
    // uhhh what does it mean for a type to have a lifetime?
    // will this make things inconvenient or something?
    pub(crate) fn into_iter_2(self) -> Box<Iterator<Item=(T,Addr<T>)>> {
        #[cfg(debug_assertions)] let sig = self.sig;
        let table: Vec<Vec<T>> = self.table.into();
        let iter = table.into_iter().enumerate()
            .flat_map(move |(t_i, bucket)|
                      bucket.into_iter()
                      .enumerate()
                      .map(move |(b_i, t)|
                           (t, Addr {
                               table: t_i,
                               bucket: b_i,
                               _type: PhantomData,
                               #[cfg(debug_assertions)] sig,
                           }))
                      );
        Box::new(iter)
    }
}

impl<T: Debug+Eq+Hash, H: Hasher+Default> AddrHashSet<T, H> {
    pub(crate) fn contains<Q: Hash+Eq>(&self, val: &Q) -> bool
        where T: Borrow<Q>
    {
        let table_index = Self::hash(val) % self.capacity;
        self.table[table_index].iter().any(|e| e.borrow() == val)
    }

    /// Locates an element by its reference or returns `None` if it's absent
    pub(crate) fn get<Q: Hash+Eq>(&self, val: &Q) -> Option<Addr<T>> 
        where T: Borrow<Q>
    {
        let table_index = Self::hash(val) % self.capacity;
        self.table[table_index].iter()
            .enumerate()
            .find(|&(_, elem)| elem.borrow() == val)
            .map(|(b_i, elem)| Addr {
                table: table_index,
                bucket: b_i,
                _type: PhantomData,
                #[cfg(debug_assertions)] sig: self.sig,
            })
    }
}

impl<T: Debug+Eq+Hash, H: Hasher+Default> Index<Addr<T>> for AddrHashSet<T, H> {
    type Output = T;
    fn index(&self, addr: Addr<T>) -> &T {
        debug_assert_eq!(self.sig, addr.sig);
        let bucket = &self.table[addr.table];
        &bucket[addr.bucket]
    }
}

impl<T: Debug+Eq+Hash, H: Hasher+Default> IndexMut<Addr<T>> for AddrHashSet<T, H> {
    fn index_mut(&mut self, addr: Addr<T>) -> &mut T {
        debug_assert_eq!(self.sig, addr.sig);
        let bucket = &mut self.table[addr.table];
        &mut bucket[addr.bucket]
    }
}



