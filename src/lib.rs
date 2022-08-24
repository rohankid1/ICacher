//! # ICacher
//! Running the same function (that return the same value) over and over again
//! can be inefficient. This lightweight, dependency-free crate attempts to
//! solve this problem by caching each return value. It will only, unless 
//! explicitly called to run multiple times or if the value isn't cached, be called once.

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
#[deprecated = "You should manually implement instead of the now deprecated traits."]
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
#[deprecated = "You should manually implement instead of the now deprecated traits."]
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
    /// Note that calling this function will clear the HashMap.
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
#[derive(Debug, Clone)]
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
    /// * You can set a capacity of the HashMap: this means that
    /// the HashMap will be able to hold a certain amount of elements
    /// without reallocating. This is memory efficient as reallocating
    /// too much can slow the program and use too much memory.  
    ///
    /// # Example
    /// Caches a closure with 2 arguments, enclosed in a
    /// tuple to simulate multiple arguments, but it is
    /// actually one.
    /// ```
    /// use icacher::ICacher;
    /// let mut adder = ICacher::new(|(a, b): (i32, i32)| a + b, Some(1));
    /// // Explicit type for `a` and `b` are needed,
    /// // but can be inferred from usage.
    #[inline]
    pub fn new(func: IFunc, capacity: Option<usize>) -> Self {
        ICacher {
            func,
            values: HashMap::with_capacity(capacity.unwrap_or_default()),
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
    /// let mut adder = ICacher::new(|(a, b)| a + b, 1);
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
    /// Note that calling this function will clear the HashMap.
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
    pub fn is_cached(&self, arg: &IType) -> bool {
        self.values.contains_key(&arg)
    }

    /// Removes a function's result and returns the result if it were found.
    ///
    /// Returns [`None`] if there weren't any found.
    ///
    /// # Example
    /// ```
    /// use icacher::ICacher;
    ///
    /// let mut multiplier = ICacher::new(|(a, b)| a * b, Some(1));
    ///
    /// let _ = multiplier.with_arg((5, 5));
    ///
    /// assert!(multiplier.is_cached(&(5, 5)));
    ///
    /// let _ = multiplier.remove_cache((5, 5));
    ///
    /// assert!(!multiplier.is_cached(&(5, 5)));
    /// ```
    #[inline]
    pub fn remove_cache(&mut self, arg: IType) -> Option<IReturn> {
        match self.values.remove(&arg) {
            Some(val) => Some(val),
            None => None,
        }
    }

    /// Same as `ICacher::with_arg`, but it
    /// does not return.
    #[inline]
    pub fn void(&mut self, arg: IType) {
        self.with_arg(arg);
    }

    /// Caches the result only if the condition is true.
    ///
    /// # Notes
    /// * If the value is already cached, it will return false
    /// * The return type signals if the value has been cached or not.
    ///
    /// # Example
    /// ```
    /// use icacher::ICacher;
    /// let mut adder = ICacher::new(|(a, b): (i32, i32)| a + b, 1);
    ///
    /// let a = 10;
    /// let b = 10;
    /// let c = 5;
    ///
    /// adder.void((a, b));
    ///
    /// let value = adder.cache_if(|| {a == b}, (a, b)); // this will not cache since it is already cached.
    ///
    /// let is_cached = adder.is_cached(&(a, b));
    ///
    /// let result = adder.cache_if(|| {is_cached}, (b, c));
    ///
    /// assert!(!value);
    /// assert!(result);
    /// ```
    #[inline]
    pub fn cache_if<Func: Fn() -> bool>(&mut self, func: Func, arg: IType) -> bool {
        if self.is_cached(&arg) || !func() {
            return false;
        }

        self.void(arg);
        return true;
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