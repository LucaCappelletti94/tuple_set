// Common test utilities and macros

/// Macro to generate tests for a tuple of specific size
#[macro_export]
macro_rules! gen_tuple_tests {
    ($mod_name:ident, ($($v:expr),+ $(,)?)) => {
        #[allow(non_snake_case)]
        mod $mod_name {
            use tuple_set::TupleSet;

            #[test]
            fn get_unchecked_success() {
                let tuple = ($($v,)+);
                let val: &i32 = unsafe { tuple.get_unchecked::<i32>() };
                assert_eq!(*val, 42);
            }

            #[test]
            #[should_panic(expected = "not found")]
            fn get_unchecked_panic() {
                let tuple = ($($v,)+);
                let _: &bool = unsafe { tuple.get_unchecked::<bool>() };
            }

            #[test]
            fn map_unchecked_success() {
                let mut tuple = ($($v,)+);
                let result = unsafe {
                    tuple.map_unchecked::<i32, _, _>(|x: &mut i32| { *x *= 2; *x })
                };
                assert_eq!(result, 84);
            }

            #[test]
            #[should_panic(expected = "not found")]
            fn map_unchecked_panic() {
                let mut tuple = ($($v,)+);
                unsafe { tuple.map_unchecked::<bool, _, _>(|x: &mut bool| *x = true); }
            }
        }
    };
}
