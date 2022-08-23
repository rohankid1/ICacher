//! # ICacher
//! This crate provides one new type (and 1 public trait for which
//! you can implement your cacher struct(s) with) which is useful for
//! optimisations (it is called [memoization](https://en.wikipedia.org/wiki/Memoization)). Running the same function (that
//! return the same value) over and over again can be inefficient.
//! This lightweight, dependency-free crate attempts solve this problem by caching
//! each return value. Only once, unless explicitly called to run
//! multiple times, it will be called.
//!
//! This crate will probably receive regular updates from time to time.
//! Updates *may* or *may not* have new features in order to keep this
//! lightweight and simple.

use std::{collections::HashMap, hash::Hash};

/// This trait provides core functionality
/// of a function cacher:
/// * `new()`
/// * `with_arg()`
pub trait FnCacher<IFunc, IType, IReturn>
where
    IFunc: Fn(IType) -> IReturn,
    IType: Clone + Hash + Eq,
    IReturn: Clone,
{
    /// Creates a new instance of the cacher struct.
    /// This method takes in a [`Fn`] closure which is stored
    /// in the struct instance.
    fn new(func: IFunc) -> Self;

    /// Returns the inner value.
    ///
    /// If the value is not found in a [`HashMap`], it will run the
    /// function, with the `arg` argument passed in, insert the value
    /// in the HashMap, and return the new value
    fn with_arg(&mut self, arg: IFunc) -> IReturn;
}

/// This trait is deprecated; however, the trait [`FnCacher`] remains
/// as to allow basic, but minimalistic, guidance.
#[deprecated]
pub trait FnCacherExt<IFunc, IType, IReturn>: FnCacher<IFunc, IType, IReturn>
where
    IFunc: Fn(IType) -> IReturn,
    IType: Clone + Hash + Eq,
    IReturn: Clone,
{
    /// Clears the HashMap.
    fn reset(&mut self);

    /// Modifies the closure of the Cacher.
    ///
    /// Note that calling this function will clear the HashMap.
    /// If you want to achieve the same but without resetting it,
    /// use the `to_unchanged()` method.
    fn to(&mut self, f: IType);

    /// Same as `to()` except that it does not change the value
    /// at all.
    fn to_unchanged(&mut self, f: IType);
}

/// This trait, as well as [`FnCacherExt`], is deprecated as extension traits
/// did not provide any benefits; methods that were in the
/// trait are now provided directly in the [`ICacher`] type.
/// 
/// The reason they are marked as deprecated and not removed,
/// is that to minimise the amount of breaking changes of other
/// codebases.
#[deprecated]
pub trait ICacherExt<IFunc, IType, IReturn>: __private::Sealed
where
    IFunc: Fn(IType) -> IReturn,
    IType: Clone + Hash + Eq,
    IReturn: Clone,
{
    /// Clears the HashMap
    fn reset(&mut self);

    /// Modifies the closure of the Cacher.
    ///
    /// Note that calling this function will reset the value to [`None`].
    /// If you want to achieve the same but without resetting it,
    /// use the `to_unchanged()` method.
    fn to(&mut self, func: IFunc);

    /// Same as `to()` except that it does not change the value
    /// at all.
    fn to_unchanged(&mut self, func: IFunc);

    // /// May or may not cache the function depending on a condition.
    // ///
    // /// If the resulting condition returns true, it will cache
    // /// the function, otherwise it will not.
    // fn cache_if<T: Clone, CondFunc: Fn(T) -> bool>(&mut self, arg: T, condition: CondFunc, func: IFunc);

    // /// May or may cache the function depending on a condition.
    // /// This function is literally the reciprocal of `cache_if`
    // ///
    // /// If the resulting condition returns false, it will cache
    // /// the function, otherwise it will not.
    // fn cache_not_if<T: Clone, CondFunc: Fn(T) -> bool>(&mut self, arg: T, condition: CondFunc, func: IFunc);
}

/// The built-in, default, generic type for caching functions and
/// storing its value in a [`HashMap`].
pub struct ICacher<IFunc, IType, IReturn>
where
    IFunc: Fn(IType) -> IReturn,
    IType: Clone + Hash + Eq,
    IReturn: Clone,
{
    func: IFunc,
    values: HashMap<IType, IReturn>,
}

impl<IFunc, IType, IReturn> ICacher<IFunc, IType, IReturn>
where
    IFunc: Fn(IType) -> IReturn,
    IType: Clone + Hash + Eq,
    IReturn: Clone,
{
    /// Creates a new [`ICacher`] instance.
    /// This takes in a closure which is expected to
    /// have at least one argument.
    ///
    /// # Notes
    /// * Use the `()` type if you do not want to return
    ///  anything.
    /// * If you need to have multiple parameters, enclose
    /// them in a tuple.
    ///
    /// # Example
    /// Caches a closure with 2 arguments, enclosed in a
    /// tuple to simulate multiple arguments, but it is
    /// actually one.
    /// ```
    /// use icacher::ICacher;
    /// let mut adder = ICacher::new(|(a, b): (i32, i32)| a + b);
    /// // Explicit type for `a` and `b` are needed,
    /// // but can be inferred from usage.
    #[inline]
    pub fn new(func: IFunc) -> Self {
        ICacher {
            func,
            values: HashMap::new(),
        }
    }

    /// Runs the closure given. If there is a value found
    /// to be in the HashMap, it will return the cached value.
    /// Otherwise, it will return a new value and cache that value
    /// by inserting it into the HashMap.
    ///
    /// # Example
    /// ```
    /// use icacher::ICacher;
    ///
    /// let mut adder = ICacher::new(|(a, b)| a + b);
    /// let value = adder.with_arg((20, 30));
    ///
    /// assert_eq!(value, 50);
    /// ```
    #[inline]
    pub fn with_arg(&mut self, arg: IType) -> IReturn {
        if self.values.contains_key(&arg) {
            return self.values[&arg].clone();
        }

        let value = (self.func)(arg.clone());
        self.values.insert(arg, value.clone());
        value
    }

    /// Clears the HashMap
    #[inline]
    pub fn reset(&mut self) {
        self.values.clear();
    }

    /// Modifies the closure of the Cacher.
    ///
    /// Note that calling this function will reset the value to [`None`].
    /// If you want to achieve the same but without resetting it,
    /// use the `to_unchanged()` method.
    #[inline]
    pub fn to(&mut self, func: IFunc) {
        self.to_unchanged(func);
        self.values.clear();
    }

    /// Same as `to()` except that it does not change the value
    /// at all.
    #[inline]
    pub fn to_unchanged(&mut self, func: IFunc) {
        self.func = func;
    }

    /// Checks if a function's result is cached.
    #[inline]
    pub fn is_cached(&self, arg: IType) -> bool {
        self.values.contains_key(&arg)
    }

    /// Removes a function's result and returns the result if it was found.
    ///
    /// Returns [`None`] if there weren't any found.
    ///
    /// # Example
    /// ```
    /// use icacher::ICacher;
    ///
    /// let mut multiplier = ICacher::new(|(a, b)| a * b);
    ///
    /// let _ = multiplier.with_arg((5, 5));
    ///
    /// assert!(multiplier.is_cached((5, 5)));
    ///
    /// let _ = multiplier.remove_cache((5, 5));
    ///
    /// assert!(!multiplier.is_cached((5, 5)));
    /// ```
    #[inline]
    pub fn remove_cache(&mut self, arg: IType) -> Option<IReturn> {
        match self.values.remove(&arg) {
            Some(val) => Some(val),
            None => None,
        }
    }
}

mod __private {
    pub trait Sealed {}

    impl<A, B, C> Sealed for super::ICacher<A, B, C>
    where
        A: Fn(B) -> C,
        B: Clone + super::Hash + Eq,
        C: Clone,
    {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caches_into_hashmap() {
        let mut cacher = ICacher::new(|a: i32| a + 1);

        cacher.with_arg(0);

        assert!(cacher.is_cached(0));
    }
}
