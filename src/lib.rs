#![doc = include_str!("../README.md")]
#![no_std]

use core::any::TypeId;

/// Trait for accessing and manipulating tuple elements by type.
pub trait TupleSet {
    /// Returns the number of times type `T` appears in the tuple.
    fn count<T: 'static>(&self) -> usize;

    /// Returns true if the tuple contains exactly one instance of type `T`.
    #[inline]
    fn contains_unique<T: 'static>(&self) -> bool {
        self.count::<T>() == 1
    }

    /// Sets the value for type `T` if it appears exactly once in the tuple.
    ///
    /// Returns `None` on success, `Some(value)` if the type is not found or
    /// appears multiple times.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuple_set::TupleSet;
    ///
    /// let mut tuple = (42i32, "hello", 3.14f64);
    /// assert!(tuple.set(100i32).is_none());
    /// assert_eq!(tuple.0, 100);
    ///
    /// // Type not found
    /// let result = tuple.set(true);
    /// assert_eq!(result, Some(true));
    /// ```
    fn set<T: 'static>(&mut self, value: T) -> Option<T> {
        if !self.contains_unique::<T>() {
            return Some(value);
        }

        unsafe {
            self.set_unchecked(value);
        }

        None
    }

    /// Get a reference to the value for type `T` in the tuple if it appears
    /// exactly once.
    ///
    /// Returns `Some(&T)` on success, or `None` if the type is not found or
    /// appears multiple times.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuple_set::TupleSet;
    ///
    /// let tuple = (42i32, "hello", 3.14f64);
    ///
    /// let value: Option<&i32> = tuple.get();
    /// assert_eq!(value, Some(&42i32));
    /// ```
    fn get<T: 'static>(&self) -> Option<&T> {
        if !self.contains_unique::<T>() {
            return None;
        }

        unsafe { Some(self.get_unchecked()) }
    }

    /// Sets the value for type `T` in the tuple without checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `T` appears exactly once in the tuple.
    /// Calling this with a type that doesn't exist or appears multiple times
    /// may lead to respectively to a panic or changing solely the first
    /// occurrence.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuple_set::TupleSet;
    ///
    /// let mut tuple = (42i32, "hello", 3.14f64);
    /// unsafe {
    ///     tuple.set_unchecked(200i32);
    /// }
    /// assert_eq!(tuple.0, 200);
    /// ```
    unsafe fn set_unchecked<T: 'static>(&mut self, value: T) {
        unsafe {
            self.map_unchecked(|x: &mut T| {
                *x = value;
            });
        }
    }

    /// Get a reference to the value for type `T` in the tuple without checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `T` appears exactly once in the tuple.
    /// Calling this with a type that doesn't exist will panic, while calling
    /// this with a type that appears multiple times will return a reference to
    /// the first occurrence.
    unsafe fn get_unchecked<T: 'static>(&self) -> &T;

    /// Applies a mapping function to the value of type `T` in the tuple.
    ///
    /// Returns `Some(result)` with the function's return value on success,
    /// or `None` if the type is not found or appears multiple times.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuple_set::TupleSet;
    ///
    /// let mut tuple = (42i32, "hello", 3.14f64);
    /// let old = tuple
    ///     .map(|x: &mut i32| {
    ///         let old = *x;
    ///         *x *= 2;
    ///         old
    ///     })
    ///     .unwrap();
    /// assert_eq!(old, 42);
    /// assert_eq!(tuple.0, 84);
    /// ```
    fn map<T: 'static, F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        if !self.contains_unique::<T>() {
            return None;
        }

        unsafe { Some(self.map_unchecked(f)) }
    }

    /// Applies a mapping function to the value of type `T` in the tuple without
    /// checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `T` appears exactly once in the tuple.
    /// Calling this with a type that doesn't exist will panic, while calling
    /// this with a type that appears multiple times may lead to solely changing
    /// the first occurrence.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuple_set::TupleSet;
    ///
    /// let mut tuple = (42i32, "hello", 3.14f64);
    /// let old = unsafe {
    ///     tuple.map_unchecked(|x: &mut i32| {
    ///         let old = *x;
    ///         *x *= 2;
    ///         old
    ///     })
    /// };
    /// assert_eq!(old, 42);
    /// assert_eq!(tuple.0, 84);
    /// ```
    unsafe fn map_unchecked<T: 'static, F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R;

    /// Takes the value out of a `T` field, leaving `T::default()` in its place.
    #[inline]
    fn take<T: 'static + Default>(&mut self) -> Option<T> {
        // Get the current value and replace with default
        self.map(core::mem::take)
    }
}

// Macro to generate implementations
macro_rules! impl_tuple_traits {
    ($($idx:tt: $T:ident),+) => {
        impl<$($T: 'static),+> TupleSet for ($($T,)+) {
			fn count<Target: 'static>(&self) -> usize {
                0 $(+ (TypeId::of::<Target>() == TypeId::of::<$T>()) as usize)+
            }

            unsafe fn get_unchecked<Target: 'static>(&self) -> &Target {
                $(
                    if TypeId::of::<Target>() == TypeId::of::<$T>() {
                        // SAFETY: We've verified Target == $T via TypeId
                        unsafe {
                            let ptr = &self.$idx as *const $T as *const Target;
                            return &*ptr;
                        }
                    }
				)+

                panic!(
                    "Type '{}' not found in tuple. This is undefined behavior.",
                    core::any::type_name::<Target>()
                );
            }

            unsafe fn map_unchecked<Target: 'static, F, R>(&mut self, f: F) -> R
            where
                F: FnOnce(&mut Target) -> R,
            {
                $(
                    if TypeId::of::<Target>() == TypeId::of::<$T>() {
                        // SAFETY: We've verified Target == $T via TypeId
                        unsafe {
                            let ptr = &mut self.$idx as *mut $T as *mut Target;
                            return f(&mut *ptr);
                        }
                    }
                )+

                panic!(
                    "Type '{}' not found in tuple. This is undefined behavior.",
                    core::any::type_name::<Target>()
                );
            }
        }
    };
}

// Recursive macro to generate all tuple implementations from 1 to N elements
// This builds up from 1-element tuples to N-element tuples
macro_rules! impl_tuple_traits_recursive {
    // Generate impl for current accumulated state, then add next element
    (@ [$($done_idx:tt: $done_T:ident),+] $idx:tt: $T:ident $(, $($rest:tt)*)?) => {
        impl_tuple_traits!($($done_idx: $done_T),+);
        impl_tuple_traits_recursive!(@ [$($done_idx: $done_T,)+ $idx: $T] $($($rest)*)?);
    };
    // Final case: generate the last impl
    (@ [$($done_idx:tt: $done_T:ident),+]) => {
        impl_tuple_traits!($($done_idx: $done_T),+);
    };
    // Entry point: start with first element in accumulator
    ($first_idx:tt: $first_T:ident $(, $($rest:tt)*)?) => {
        impl_tuple_traits_recursive!(@ [$first_idx: $first_T] $($($rest)*)?);
    };
}

// Generate implementations for tuples up to 128 elements
impl_tuple_traits_recursive!(
    0: T1, 1: T2, 2: T3, 3: T4, 4: T5, 5: T6, 6: T7, 7: T8,
    8: T9, 9: T10, 10: T11, 11: T12, 12: T13, 13: T14, 14: T15, 15: T16,
    16: T17, 17: T18, 18: T19, 19: T20, 20: T21, 21: T22, 22: T23, 23: T24,
    24: T25, 25: T26, 26: T27, 27: T28, 28: T29, 29: T30, 30: T31, 31: T32,
    32: T33, 33: T34, 34: T35, 35: T36, 36: T37, 37: T38, 38: T39, 39: T40,
    40: T41, 41: T42, 42: T43, 43: T44, 44: T45, 45: T46, 46: T47, 47: T48,
    48: T49, 49: T50, 50: T51, 51: T52, 52: T53, 53: T54, 54: T55, 55: T56,
    56: T57, 57: T58, 58: T59, 59: T60, 60: T61, 61: T62, 62: T63, 63: T64
);
