//! fn<T>(T) -> T

/// The "standard" definition of identity.
pub fn id<T>(x: T) -> T {
    x
}

/// identity with an effect.
pub fn effectful_id<T>(x: T) -> T {
    println!("oh no");
    x
}

/// diverging "identity" that panics!
pub fn panicking_id<T>(_: T) -> T {
    panic!("at the disco")
}

/// the never-ending "identity"
pub fn diverging_id<T>(_: T) -> T {
    loop {}
}
