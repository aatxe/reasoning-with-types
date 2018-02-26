//! fn<T>(T) -> T where T: Display

use std::fmt::Display;

/// The "standard" definition of identity.
pub fn id<T>(x: T) -> T where T: Display {
    x
}

/// identity with an effect.
pub fn effectful_id<T>(x: T) -> T where T: Display {
    println!("oh no");
    x
}

/// diverging "identity" that panics!
pub fn panicking_id<T>(_: T) -> T where T: Display {
    panic!("at the disco")
}

/// the never-ending "identity"
pub fn diverging_id<T>(_: T) -> T where T: Display {
    loop {}
}

// identity, but it prints the value along the way
pub fn trace<T>(x: T) -> T where T: Display {
    println!("{}", x);
    x
}
