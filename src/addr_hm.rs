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

extern crate rand;

use std::fmt::Debug;
use std::borrow::Borrow;
use std::ops::{Index, IndexMut};
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::marker::PhantomData;
use std::vec;

use vertex::NodeT;

const RESIZE_THRESHOLD: f64 = 1.5;
const RESIZE_FACTOR: f64 = 2.0;
const MAX_BUCKET_SIZE: usize = 32;
const DEFAULT_TABLE_CAPACITY: usize = 32;
const DEFAULT_BUCKET_CAPACITY: usize = 4;

pub(crate) trait AddrType: Clone + Eq + Hash {
    #[cfg(debug_assertions)]
    fn matches_signature(&self, other: Signature) -> bool;
    #[cfg(debug_assertions)]        fn new(usize, usize, Signature) -> Self;
    #[cfg(not(debug_assertions))]   fn new(usize, usize) -> Self;
    fn get_table(&self) -> usize;
    fn get_bucket(&self) -> usize;
}

// TODO make not pub
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)] pub struct VertAddr(GenericAddr);
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)] pub struct EdgeAddr(GenericAddr);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct GenericAddr {
    table: usize,
    bucket: usize,
    #[cfg(debug_assertions)] sig: Signature,
}

impl AddrType for GenericAddr {
    #[cfg(debug_assertions)]
    fn matches_signature(&self, other: Signature) -> bool { self.sig == other }
    #[cfg(debug_assertions)] 
    fn new(t: usize, b: usize, s: Signature) -> Self {
        GenericAddr { table: t, bucket: b, sig: s }
    }
    #[cfg(not(debug_assertions))]
    fn new(t: usize, b: usize) -> Self { GenericAddr { table: t, bucket: b } }
    fn get_table(&self) -> usize { self.table } 
    fn get_bucket(&self) -> usize { self.bucket }
}

impl AddrType for VertAddr {
    #[cfg(debug_assertions)]
    fn matches_signature(&self, other: Signature) -> bool { 
        self.0.matches_signature(other) 
    }
    #[cfg(debug_assertions)]
    fn new(t: usize, b: usize, s: Signature) -> Self {
        VertAddr(GenericAddr::new(t, b, s))
    }
    #[cfg(not(debug_assertions))]
    fn new(t: usize, b: usize) -> Self { VertAddr(GenericAddr::new(t,b)) }
    fn get_table(&self) -> usize { self.0.get_table() }
    fn get_bucket(&self) -> usize { self.0.get_bucket() }
}

impl AddrType for EdgeAddr {
    #[cfg(debug_assertions)]
    fn matches_signature(&self, other: Signature) -> bool { 
        self.0.matches_signature(other) 
    }
    #[cfg(debug_assertions)]
    fn new(t: usize, b: usize, s: Signature) -> Self {
        EdgeAddr(GenericAddr::new(t, b, s))
    }
    #[cfg(not(debug_assertions))]
    fn new(t: usize, b: usize, s: Signature) -> Self { EdgeAddr(GenericAddr::new(t, b)) }
    fn get_table(&self) -> usize { self.0.get_table() }
    fn get_bucket(&self) -> usize { self.0.get_bucket() }
}


#[cfg(debug_assertions)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) struct Signature(usize);

#[cfg(debug_assertions)]
impl Signature {
    fn random() -> Self { Signature(rand::random()) }
}

pub(crate) type AddrSet<T,A> = AddrHashSet<T, DefaultHasher, A>;

#[derive(Debug)]
pub(crate) struct AddrHashSet<T: NodeT, H: Hasher+Default, A: AddrType> {
    capacity: usize,
    size: usize,
    table: Box<[Vec<T>]>,
    _hasher: PhantomData<H>,
    _addr: PhantomData<A>,
    #[cfg(debug_assertions)] sig: Signature,
}

impl<T: NodeT, A: AddrType> Default for AddrHashSet<T, DefaultHasher, A> {
    fn default() -> Self {
        AddrHashSet::with_capacity(DEFAULT_TABLE_CAPACITY)
    }
}

impl<T: NodeT, A: AddrType> AddrHashSet<T, DefaultHasher, A> {
    pub(crate) fn with_capacity(c: usize) -> Self {
        Self::with_capacity_and_hasher(c)
    }

}

impl<T: NodeT, H: Hasher+Default, A: AddrType> AddrHashSet<T, H, A> {
    /// Create new `AddrHashSet` with specified capacity and hasher
    pub(crate) fn with_capacity_and_hasher(c: usize) -> Self {
        // better way?
        let v: Vec<Vec<T>> = (0..c).map(|_| vec![]).collect();
        AddrHashSet {
            size: 0,
            capacity: DEFAULT_TABLE_CAPACITY,
            table: v.into_boxed_slice(),
            _hasher: PhantomData,
            _addr: PhantomData,
            #[cfg(debug_assertions)] sig: Signature::random(),
        }
    }
    /// Create a new, larger `AddrHashSet` and copy the data over
    /// Return a translation map of old `Addr`s to new ones
    // TODO do not copy over "deleted" elements?
    pub(crate) fn from_old<AA: AddrType>(old: AddrHashSet<T,H,AA>) -> (Self, HashMap<AA,A>) {
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
    pub(crate) fn insert(&mut self, val: T) -> Option<A> {
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
        #[cfg(not(debug_assertions))] return Some(A::new(table_index, bucket_index));
        #[cfg(debug_assertions)] Some(A::new(table_index, bucket_index, self.sig))
    }


    pub(crate) fn iter<'a>(&'a self) -> Box<Iterator<Item=(&'a T, A)>+'a> {
        let sig = self.sig;
        let iter = self.table.iter().enumerate()
            .flat_map(move |(t_i, bucket)|
                      bucket.iter()
                      .enumerate()
                      .map(move |(b_i, t)| {
                           let a = A::new(t_i, b_i, #[cfg(debug_assertions)] sig);
                           (t, a)
                      }));
        Box::new(iter)
    }

    pub(crate) fn into_iter_1(self) -> vec::IntoIter<(T,A)> {
        // Note: this collects to a Vec first
        #[cfg(debug_assertions)] let sig = self.sig;
        let table: Vec<Vec<T>> = self.table.into();
        let elems: Vec<(T,A)> = table.into_iter().enumerate()
            .flat_map(|(t_i, bucket)| 
                      bucket.into_iter()
                      .enumerate()
                      .map(move |(b_i, t)| {
                           #[cfg(debug_assertions)] return (t, A::new(t_i, b_i, sig));
                           #[cfg(not(debug_assertions))] (t, A::new(t_i, b_i))
                           })
                      )
            .collect();
        elems.into_iter()
    }

}

impl<T: 'static + NodeT, H: Hasher+Default, A: AddrType> AddrHashSet<T, H, A> {
    // uhhh what does it mean for a type to have a lifetime?
    // will this make things inconvenient or something?
    pub(crate) fn into_iter_2(self) -> Box<Iterator<Item=(T,A)>> {
        #[cfg(debug_assertions)] let sig = self.sig;
        let table: Vec<Vec<T>> = self.table.into();
        let iter = table.into_iter().enumerate()
            .flat_map(move |(t_i, bucket)|
                      bucket.into_iter()
                      .enumerate()
                      .map(move |(b_i, t)|
                           (t, A::new(t_i, b_i, #[cfg(debug_assertions)] sig))
                           ));
        Box::new(iter)
    }
}

impl<T: NodeT, H: Hasher+Default, A: AddrType> AddrHashSet<T, H, A> {
    pub(crate) fn contains<Q: Hash+Eq>(&self, val: &Q) -> bool
        where T: Borrow<Q>
    {
        let table_index = Self::hash(val) % self.capacity;
        self.table[table_index].iter().any(|e| e.borrow() == val)
    }

    /// Locates an element by its reference or returns `None` if it's absent
    pub(crate) fn get<Q: Hash+Eq>(&self, val: &Q) -> Option<A> 
        where T: Borrow<Q>
    {
        let table_index = Self::hash(val) % self.capacity;
        let sig = self.sig;
        self.table[table_index].iter()
            .enumerate()
            .find(|&(_, elem)| elem.borrow() == val)
            .map(|(b_i, elem)| A::new(table_index, b_i, #[cfg(debug_assertions)] sig))
    }
}

impl<T: NodeT, H: Hasher+Default, A: AddrType> Index<A> for AddrHashSet<T, H, A> {
    type Output = T;
    fn index(&self, addr: A) -> &T {
        debug_assert!(addr.matches_signature(self.sig));
        let bucket = &self.table[addr.get_table()];
        &bucket[addr.get_bucket()]
    }
}

impl<T: NodeT, H: Hasher+Default, A: AddrType> IndexMut<A> for AddrHashSet<T, H, A> {
    fn index_mut(&mut self, addr: A) -> &mut T {
        debug_assert!(addr.matches_signature(self.sig));
        let bucket = &mut self.table[addr.get_table()];
        &mut bucket[addr.get_bucket()]
    }
}



