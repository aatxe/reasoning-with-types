//! Noninteference for Free
#![allow(unused_imports)]

pub struct LocalSecret<T>(T);

// this compiles and violates noninterference.
pub fn unwrap_secret<T>(secret: LocalSecret<T>) -> T {
    secret.0
}


// This needs to be encapsulated in its own module, otherwise we could violate noninterference.
pub mod secret {
    pub struct Secret<T>(T);
}
use self::secret::Secret;

// This fails to compile.
// pub fn unwrap_secret<T>(secret: Secret<T>) -> T {
//     secret.0
// }

pub mod secret2 {
    #[derive(Copy, Clone, Default)]
    pub struct Secret<T>(T);

    use std::ops::{Add, Sub};

    impl<T> Add for Secret<T> where T: Add {
        type Output = Secret<<T as Add>::Output>;

        fn add(self, other: Secret<T>) -> Self::Output {
            Secret(self.0 + other.0)
        }
    }

    impl<T> Sub for Secret<T> where T: Sub {
        type Output = Secret<<T as Sub>::Output>;

        fn sub(self, other: Secret<T>) -> Self::Output {
            Secret(self.0 - other.0)
        }
    }

    impl Secret<bool> {
        pub fn branch<F, T>(&self, cons: F, alt: F) -> Secret<T> where F: Fn() -> Secret<T> {
            if self.0 {
                cons()
            } else {
                alt()
            }
        }
    }

    // ...
}
