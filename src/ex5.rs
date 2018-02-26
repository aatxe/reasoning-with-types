//! Relaxed Noninterference for Free
#![allow(unused_variables)]

use std::fmt::{Debug, Error, Formatter};
use std::hash::{Hash, Hasher};

pub struct Secret<T>(T);

// Empty declassification policy: do nothing.
// In practice, we'd probably add all of the secret-combining stuff to this trait.
pub trait Sec<T>: private::Sealed {}
impl<T> Sec<T> for Secret<T> {}

// This module is used to seal `Sec<T>` preventing any further implementations.
mod private {
    use super::Secret;
    pub trait Sealed {}
    impl<T> Sealed for Secret<T> {}
}

pub fn f<S, T>(x: u32, y: S) -> u32 where S: Sec<u32> {
    // the folowing line is not legal...
    // return y.0;
    x
}

// Debug declassification policy: can format the value for debugging purposes
impl<T> Debug for Secret<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

// Zeroable declassification policy: can determine whether or not this is zero
// n.b. this trait is nightly-only, but I've inlined it here for simplicity.
pub trait Zeroable {
    fn is_zero(&self) -> bool;
}

impl<T> Zeroable for Secret<T> where T: Zeroable {
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

// Hash declassification policy: can compute a hash of the value
// For real security purposes, we'd probably want a more strict version that uses only a specific
// cryptographic hash (like SHA256 or SHA512).
impl<T> Hash for Secret<T> where T: Hash {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.0.hash(state);
    }
}

pub fn check<'a, S>(password: S, db_hash: u64) -> bool where S: Sec<&'a str> + Hash {
    // please don't actually do this, use SHA256 instead.
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    hasher.finish() == db_hash
}
