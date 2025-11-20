#![doc = include_str!("../README.md")]
#![no_std]

use core::any::TypeId;

/// Trait for setting values in tuples.
pub trait TupleSet<T> {
    /// Returns the number of times type `T` appears in the tuple.
    fn count(&self) -> usize;

    /// Returns true if the tuple contains exactly one instance of type `T`.
    #[inline]
    fn contains_unique(&self) -> bool {
        self.count() == 1
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
    fn set(&mut self, value: T) -> Option<T> {
        if !self.contains_unique() {
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
    /// let value = tuple.get();
    /// assert_eq!(value, Some(&42i32));
    /// ```
    fn get(&self) -> Option<&T> {
        if !self.contains_unique() {
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
    unsafe fn set_unchecked(&mut self, value: T) {
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
    unsafe fn get_unchecked(&self) -> &T;

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
    fn map<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        if !self.contains_unique() {
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
    unsafe fn map_unchecked<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R;

    /// Takes the value out of a `T` field, leaving `T::default()` in its place.
    #[inline]
    fn take(&mut self) -> Option<T>
    where
        T: Default,
    {
        // Get the current value and replace with default
        self.map(core::mem::take)
    }
}

// Macro to generate implementations
macro_rules! impl_tuple_traits {
    ($($idx:tt: $T:ident),+) => {
        impl<Target: 'static, $($T: 'static),+> TupleSet<Target> for ($($T,)+) {
			fn count(&self) -> usize {
                let mut count = 0;
                $(
                    if TypeId::of::<Target>() == TypeId::of::<$T>() {
                        count += 1;
                    }
                )+
                count
            }

            unsafe fn get_unchecked(&self) -> &Target {
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

            unsafe fn map_unchecked<F, R>(&mut self, f: F) -> R
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

// Macro to recursively generate tests for tuples of decreasing sizes
#[cfg(test)]
macro_rules! gen_tuple_tests_recursive {
    // Base case: single element tuple (first element is always i32 with value 42)
    ($mod_name:ident; $v0:expr) => {
        #[allow(non_snake_case)]
        mod $mod_name {
            use super::*;

            #[test]
            fn get_unchecked_success() {
                let tuple = ($v0,);
                let val: &i32 = unsafe { tuple.get_unchecked() };
                assert_eq!(*val, 42);
            }

            #[test]
            #[should_panic(expected = "not found")]
            fn get_unchecked_panic() {
                let tuple = ($v0,);
                let _: &bool = unsafe { tuple.get_unchecked() };
            }

            #[test]
            fn map_unchecked_success() {
                let mut tuple = ($v0,);
                let result = unsafe {
                    tuple.map_unchecked(|x: &mut i32| { *x *= 2; *x })
                };
                assert_eq!(result, 84);
            }

            #[test]
            #[should_panic(expected = "not found")]
            fn map_unchecked_panic() {
                let mut tuple = ($v0,);
                unsafe { tuple.map_unchecked(|x: &mut bool| *x = true); }
            }
        }
    };
    // Recursive case: generate test for current size, then recurse to smaller size
    ($mod_name:ident; $v0:expr, $($v:expr),+; $next_mod:ident; $($rest:tt)*) => {
        #[allow(non_snake_case)]
        mod $mod_name {
            use super::*;

            #[test]
            fn get_unchecked_success() {
                let tuple = ($v0, $($v),+);
                let val: &i32 = unsafe { tuple.get_unchecked() };
                assert_eq!(*val, 42);
            }

            #[test]
            #[should_panic(expected = "not found")]
            fn get_unchecked_panic() {
                let tuple = ($v0, $($v),+);
                let _: &bool = unsafe { tuple.get_unchecked() };
            }

            #[test]
            fn map_unchecked_success() {
                let mut tuple = ($v0, $($v),+);
                let result = unsafe {
                    tuple.map_unchecked(|x: &mut i32| { *x *= 2; *x })
                };
                assert_eq!(result, 84);
            }

            #[test]
            #[should_panic(expected = "not found")]
            fn map_unchecked_panic() {
                let mut tuple = ($v0, $($v),+);
                unsafe { tuple.map_unchecked(|x: &mut bool| *x = true); }
            }
        }
        gen_tuple_tests_recursive!($next_mod; $($rest)*);
    };
}

// Generate tests for tuples from size 10 down to size 1
#[cfg(test)]
gen_tuple_tests_recursive!(
    gen_test_10; 42i32, 1i64, 2i64, 3i64, 4i64, 5i64, 6i64, 7i64, 8i64, 9i64;
    gen_test_9; 42i32, 1i64, 2i64, 3i64, 4i64, 5i64, 6i64, 7i64, 8i64;
    gen_test_8; 42i32, 1i64, 2i64, 3i64, 4i64, 5i64, 6i64, 7i64;
    gen_test_7; 42i32, 1i64, 2i64, 3i64, 4i64, 5i64, 6i64;
    gen_test_6; 42i32, 1i64, 2i64, 3i64, 4i64, 5i64;
    gen_test_5; 42i32, 1i64, 2i64, 3i64, 4i64;
    gen_test_4; 42i32, 1i64, 2i64, 3i64;
    gen_test_3; 42i32, 1i64, 2i64;
    gen_test_2; 42i32, 1i64;
    gen_test_1; 42i32
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let tuple = (42i32, "hello", 2.5f64);
        assert_eq!(<(i32, &str, f64) as TupleSet<i32>>::count(&tuple), 1);
        assert_eq!(<(i32, &str, f64) as TupleSet<bool>>::count(&tuple), 0);
    }

    #[test]
    fn test_count_duplicates() {
        let tuple = (42i32, "hello", 100i32);
        assert_eq!(<(i32, &str, i32) as TupleSet<i32>>::count(&tuple), 2);
    }

    #[test]
    fn test_set_returns_none_when_found() {
        let mut tuple = (42i32, "hello", 2.5f64);
        assert!(tuple.set(100i32).is_none());
        assert_eq!(tuple.0, 100);
    }

    #[test]
    fn test_set_returns_value_when_not_found() {
        let mut tuple = (42i32, "hello", 2.5f64);
        let result = tuple.set(true);
        assert_eq!(result, Some(true));
    }

    #[test]
    fn test_set_returns_value_when_duplicates() {
        let mut tuple = (42i32, "hello", 100i32);
        assert_eq!(tuple.set(200i32), Some(200));
    }

    #[test]
    fn test_set_unchecked_with_duplicates() {
        let mut tuple = (42i32, 100i32, "hello");
        unsafe {
            tuple.set_unchecked(82i32);
        }
        assert_eq!(tuple.0, 82);
        assert_eq!(tuple.1, 100);
    }

    #[test]
    fn test_map_returns_none_when_duplicates() {
        let mut tuple = (42i32, 100i32, "hello");
        assert_eq!(tuple.map(|_x: &mut i32| 200), None);
    }

    #[test]
    fn test_map_returns_none_when_not_found() {
        let mut tuple = (42i32, "hello", 2.5f64);
        assert_eq!(tuple.map(|_x: &mut bool| true), None);
    }

    #[test]
    fn test_map_unchecked_with_duplicates() {
        let mut tuple = (10i32, 20i32, "test");
        let result = unsafe {
            tuple.map_unchecked(|x: &mut i32| {
                *x += 1;
                *x
            })
        };
        assert_eq!(result, 11);
        assert_eq!(tuple.0, 11);
        assert_eq!(tuple.1, 20);
    }

    #[test]
    fn test_option_helpers() {
        let mut tuple = (Some(42i32), "hello", Some(2.5f64));
        tuple.set(Some(100));
        assert_eq!(tuple.0, Some(100));

        let value: Option<Option<f64>> = tuple.take();
        assert_eq!(value, Some(Some(2.5)));
        assert_eq!(tuple.2, None);
    }
}
