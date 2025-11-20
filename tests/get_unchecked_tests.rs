use tuple_set::TupleSet;

#[test]
fn test_get_unchecked_simple() {
    let tuple = (42i32, "hello", 2.5f64);
    let value: &i32 = unsafe { tuple.get_unchecked() };
    assert_eq!(*value, 42);
}

#[test]
#[should_panic(expected = "not found")]
fn test_get_unchecked_panic_not_found() {
    let tuple = (42i32, "hello", 2.5f64);
    let _: &bool = unsafe { tuple.get_unchecked() };
}

#[test]
fn test_get_unchecked_multiple_accesses() {
    let tuple = (42i32, "test", 2.5f64);
    let value: &i32 = unsafe { tuple.get_unchecked() };
    assert_eq!(*value, 42);
    let value: &&str = unsafe { tuple.get_unchecked() };
    assert_eq!(*value, "test");
    let value: &f64 = unsafe { tuple.get_unchecked() };
    assert_eq!(*value, 2.5);
}

#[test]
fn test_get_unchecked_with_duplicates() {
    let tuple = (42i32, 100i32, "hello");
    // Should get the first occurrence
    let value: &i32 = unsafe { tuple.get_unchecked() };
    assert_eq!(*value, 42);
}

#[test]
fn test_get_unchecked_single_element() {
    let tuple = (999i32,);
    let value: &i32 = unsafe { tuple.get_unchecked() };
    assert_eq!(*value, 999);
}

#[test]
fn test_get_unchecked_large_tuple() {
    let tuple = (1i32, 2u32, 3i64, 4u64, 5f32, 6f64, true, 'x');
    let value: &bool = unsafe { tuple.get_unchecked() };
    assert!(*value);

    let value: &char = unsafe { tuple.get_unchecked() };
    assert_eq!(*value, 'x');
}
