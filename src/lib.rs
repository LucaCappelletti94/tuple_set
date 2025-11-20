#![doc = include_str!("../README.md")]
#![no_std]

use core::any::TypeId;

/// Trait to check if a tuple contains a specific type and count occurrences.
pub trait TupleContains<T> {
    /// Returns the number of times type `T` appears in the tuple.
    fn count(&self) -> usize;

    /// Returns true if the tuple contains exactly one instance of type `T`.
    #[inline]
    fn contains_unique(&self) -> bool {
        self.count() == 1
    }
}

/// Trait for setting values in tuples.
pub trait TupleSet<T> {
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
    fn set(&mut self, value: T) -> Option<T>;

    /// Get a reference to the value for type `T` in the tuple if it appears
    /// exactly once.
    fn get(&self) -> Option<&T>;

    /// Sets the value for type `T` in the tuple without checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `T` appears exactly once in the tuple.
    /// Calling this with a type that doesn't exist or appears multiple times
    /// will panic.
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
    unsafe fn set_unchecked(&mut self, value: T);

    /// Get a reference to the value for type `T` in the tuple without checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `T` appears exactly once in the tuple.
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
        F: FnOnce(&mut T) -> R;

    /// Applies a mapping function to the value of type `T` in the tuple without
    /// checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `T` appears exactly once in the tuple.
    /// Calling this with a type that doesn't exist or appears multiple times
    /// will panic.
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
        // TupleContains implementation
        impl<Target: 'static, $($T: 'static),+> TupleContains<Target> for ($($T,)+) {
            fn count(&self) -> usize {
                let mut count = 0;
                $(
                    if TypeId::of::<Target>() == TypeId::of::<$T>() {
                        count += 1;
                    }
                )+
                count
            }
        }

        // TupleSet implementation
        impl<Target: 'static, $($T: 'static),+> TupleSet<Target> for ($($T,)+) {
            fn set(&mut self, value: Target) -> Option<Target> {
                // Only set if exactly one match
                if <Self as TupleContains<Target>>::contains_unique(self) {
                    $(
                        if TypeId::of::<Target>() == TypeId::of::<$T>() {
                            // SAFETY: We've verified Target == $T via TypeId
                            unsafe {
                                let ptr = &mut self.$idx as *mut $T as *mut Target;
                                core::ptr::write(ptr, value);
                            }
                            return None;
                        }
                    )+
                    unreachable!()
                } else {
                    Some(value)
                }
            }

            unsafe fn set_unchecked(&mut self, value: Target) {
                $(
                    if TypeId::of::<Target>() == TypeId::of::<$T>() {
                        // SAFETY: We've verified Target == $T via TypeId
                        unsafe {
                            let ptr = &mut self.$idx as *mut $T as *mut Target;
                            core::ptr::write(ptr, value);
                        }
                        return;
                    }
                    )+

                panic!(
                    "set_unchecked: Type '{}' not found in tuple. This is undefined behavior.",
                    core::any::type_name::<Target>()
                );
            }

            fn get(&self) -> Option<&Target> {
                // Only get if exactly one match
                if <Self as TupleContains<Target>>::contains_unique(self) {
                    $(
                        if TypeId::of::<Target>() == TypeId::of::<$T>() {
                            // SAFETY: We've verified Target == $T via TypeId
                            unsafe {
                                let ptr = &self.$idx as *const $T as *const Target;
                                return Some(&*ptr);
                            }
                        }
                    )+
                    unreachable!()
                } else {
                    None
                }
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
                    "get_unchecked: Type '{}' not found in tuple. This is undefined behavior.",
                    core::any::type_name::<Target>()
                );
            }

            fn map<F, R>(&mut self, f: F) -> Option<R>
            where
                F: FnOnce(&mut Target) -> R,
            {
                // Only map if exactly one match
                if <Self as TupleContains<Target>>::contains_unique(self) {
                    $(
                        if TypeId::of::<Target>() == TypeId::of::<$T>() {
                            // SAFETY: We've verified Target == $T via TypeId
                            unsafe {
                                let ptr = &mut self.$idx as *mut $T as *mut Target;
                                let result = f(&mut *ptr);
                                return Some(result);
                            }
                        }
                    )+
                    unreachable!()
                } else {
                    None
                }
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
                    "map_unchecked: Type '{}' not found in tuple. This is undefined behavior.",
                    core::any::type_name::<Target>()
                );
            }
        }
    };
} // Recursive macro to generate all tuple implementations from 1 to N elements
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let tuple = (42i32, "hello", 2.5f64);
        let count = <(i32, &str, f64) as TupleContains<i32>>::count(&tuple);
        assert_eq!(count, 1);

        let count = <(i32, &str, f64) as TupleContains<bool>>::count(&tuple);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_count_duplicates() {
        let tuple = (42i32, "hello", 100i32);
        let count = <(i32, &str, i32) as TupleContains<i32>>::count(&tuple);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_set_single_element() {
        let mut tuple = (42i32,);
        let result = tuple.set(100i32);
        assert!(result.is_none());
        assert_eq!(tuple.0, 100);
    }

    #[test]
    fn test_set_first_position() {
        let mut tuple = (42i32, "hello", 2.5f64);
        let result = tuple.set(100i32);
        assert!(result.is_none());
        assert_eq!(tuple.0, 100);
        assert_eq!(tuple.1, "hello");
        assert_eq!(tuple.2, 2.5);
    }

    #[test]
    fn test_set_middle_position() {
        let mut tuple = (42i32, "hello", 2.5f64);
        let result = tuple.set("world");
        assert!(result.is_none());
        assert_eq!(tuple.0, 42);
        assert_eq!(tuple.1, "world");
        assert_eq!(tuple.2, 2.5);
    }

    #[test]
    fn test_set_last_position() {
        let mut tuple = (42i32, "hello", 2.5f64);
        let result = tuple.set(1.5f64);
        assert!(result.is_none());
        assert_eq!(tuple.0, 42);
        assert_eq!(tuple.1, "hello");
        assert_eq!(tuple.2, 1.5);
    }

    #[test]
    fn test_set_not_found() {
        let mut tuple = (42i32, "hello", 2.5f64);
        let result = tuple.set(true);
        assert!(result.is_some());
        assert!(result.unwrap());
        assert_eq!(tuple.0, 42); // Tuple unchanged
    }

    #[test]
    fn test_set_duplicates() {
        let mut tuple = (42i32, "hello", 100i32);
        let result = tuple.set(200i32);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 200);
        assert_eq!(tuple.0, 42); // Tuple unchanged
    }

    #[test]
    fn test_set_larger_tuple() {
        let mut tuple = (1i32, 2u32, 3i64, 4u64, 5f32, 6f64, true, 'x');
        let result = tuple.set(false);
        assert!(result.is_none());
        assert_eq!(tuple.0, 1);
        assert_eq!(tuple.1, 2);
        assert_eq!(tuple.2, 3);
        assert_eq!(tuple.3, 4);
        assert_eq!(tuple.4, 5.0);
        assert_eq!(tuple.5, 6.0);
        assert!(!tuple.6);
        assert_eq!(tuple.7, 'x');
    }

    #[test]
    fn test_set_with_i64() {
        let mut tuple = (42i32, 100i64, 2.5f64);
        let result = tuple.set(200i64);
        assert!(result.is_none());
        assert_eq!(tuple.0, 42);
        assert_eq!(tuple.1, 200);
        assert_eq!(tuple.2, 2.5);
    }

    #[test]
    fn test_set_unchecked() {
        let mut tuple = (42i32, "hello", 2.5f64);
        unsafe {
            tuple.set_unchecked(100i32);
        }
        assert_eq!(tuple.0, 100);
    }

    #[test]
    #[should_panic(expected = "set_unchecked: Type")]
    fn test_set_unchecked_panic() {
        let mut tuple = (42i32, "hello", 2.5f64);
        unsafe {
            tuple.set_unchecked(true);
        }
    }

    #[test]
    fn test_map() {
        let mut tuple = (42i32, "hello", 2.5f64);

        // Double the i32 value and return the new value
        let result = tuple.map(|x: &mut i32| {
            *x *= 2;
            *x
        });
        assert_eq!(result, Some(84));
        assert_eq!(tuple.0, 84);
    }

    #[test]
    fn test_map_with_char() {
        let mut tuple = (42i32, 'a', 2.5f64);

        // Increment the char and return the old value
        let result = tuple.map(|c: &mut char| {
            let old = *c;
            *c = 'b';
            old
        });
        assert_eq!(result, Some('a'));
        assert_eq!(tuple.1, 'b');
    }

    #[test]
    fn test_map_not_found() {
        let mut tuple = (42i32, "hello", 2.5f64);
        let result = tuple.map(|_x: &mut bool| true);
        assert_eq!(result, None);
    }

    #[test]
    fn test_map_duplicates() {
        let mut tuple = (42i32, 100i32, "hello");
        let result = tuple.map(|_x: &mut i32| 200);
        assert_eq!(result, None);
    }

    #[test]
    fn test_option_helpers() {
        let mut tuple = (Some(42i32), "hello", Some(2.5f64));

        // Use set to replace Some(42) with Some(100)
        let result = tuple.set(Some(100));
        assert!(result.is_none());
        assert_eq!(tuple.0, Some(100));

        // Use take to extract the f64 Option value (which replaces it with None since
        // Option<f64>: Default)
        let value: Option<Option<f64>> = tuple.take();
        assert_eq!(value, Some(Some(2.5)));
        assert_eq!(tuple.2, None);

        // Use set to set the None field back
        let result = tuple.set(Some(1.5));
        assert!(result.is_none());
        assert_eq!(tuple.2, Some(1.5));

        // Use map with Option methods directly when needed
        let old_val = tuple.map(|opt: &mut Option<i32>| *opt.get_or_insert(999)).unwrap();
        assert_eq!(old_val, 100);
    }

    #[test]
    fn test_64_element_tuple() {
        // Create a 64-element tuple with various types
        #[allow(clippy::type_complexity)]
        let mut tuple: (
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            f64,
            bool,
        ) = (
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 2.5, false,
        );

        // Test setting f64 value at position 62
        let result = tuple.set(1.5f64);
        assert!(result.is_none());
        assert_eq!(tuple.62, 1.5);

        // Test setting bool value at position 63
        let result = tuple.set(true);
        assert!(result.is_none());
        assert!(tuple.63);

        // Test map on the bool field
        let result = tuple.map(|b: &mut bool| {
            let old = *b;
            *b = false;
            old
        });
        assert_eq!(result, Some(true));
        assert!(!tuple.63);

        // Test counting i32 occurrences (should be 62)
        type LargeTuple = (
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            f64,
            bool,
        );
        let count = <LargeTuple as TupleContains<i32>>::count(&tuple);
        assert_eq!(count, 62);
    }
}
