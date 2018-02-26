//! fn<T>(Vec<T>) -> Vec<T>

// returns the the list without its first element
pub fn tail<T>(vec: Vec<T>) -> Vec<T> {
    vec.into_iter().skip(1).collect()
}

// returns the list backwards
pub fn reverse<T>(vec: Vec<T>) -> Vec<T> {
    let init = Vec::with_capacity(vec.capacity());
    vec.into_iter().fold(init, |mut acc, elem| {
        acc.insert(0, elem);
        acc
    })
}

// swaps the first two elements of the list if possible
pub fn swap_first_two<T>(mut vec: Vec<T>) -> Vec<T> {
    if vec.len() < 2 {
        return vec;
    }
    let elem = vec.remove(1);
    vec.insert(0, elem);
    vec
}
