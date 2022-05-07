/// cons(a, b) constructs a pair, and car(pair) and cdr(pair) returns the
/// first and last element of that pair. For example, car(cons(3, 4))
/// returns 3, and cdr(cons(3, 4)) returns 4.
///
/// Given this implementation of cons:
///
/// ```py
/// def cons(a, b):
///     def pair(f):
///         return f(a, b)
///     return pair
/// ```
/// 
/// Implement car and cdr.
pub fn cons<T>(a: T, b: T) -> impl FnOnce(&dyn Fn(T, T) -> T) -> T {
    move |f| f(a, b)
}

pub fn car<T>(f: impl FnOnce(&dyn Fn(T, T) -> T) -> T) -> T {
    f(&|a, _| a)
}

pub fn cdr<T>(f: impl FnOnce(&dyn Fn(T, T) -> T) -> T) -> T {
    f(&|_, b| b)
}
