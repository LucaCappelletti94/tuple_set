use tuple_set::TupleSet;

#[test]
fn test_set_single_element() {
    let mut tuple = (42i32,);
    assert!(tuple.set(100i32).is_none());
    assert_eq!(tuple.0, 100);
}

#[test]
fn test_set_first_position() {
    let mut tuple = (42i32, "hello", 2.5f64);
    assert!(tuple.set(100i32).is_none());
    assert_eq!(tuple.0, 100);
    assert_eq!(tuple.1, "hello");
    assert_eq!(tuple.2, 2.5);
}

#[test]
fn test_set_middle_position() {
    let mut tuple = (42i32, "hello", 2.5f64);
    assert!(tuple.set("world").is_none());
    assert_eq!(tuple.0, 42);
    assert_eq!(tuple.1, "world");
    assert_eq!(tuple.2, 2.5);
}

#[test]
fn test_set_last_position() {
    let mut tuple = (42i32, "hello", 2.5f64);
    assert!(tuple.set(9.99f64).is_none());
    assert_eq!(tuple.0, 42);
    assert_eq!(tuple.1, "hello");
    assert_eq!(tuple.2, 9.99);
}

#[test]
fn test_set_returns_value_when_not_found() {
    let mut tuple = (42i32, "hello", 2.5f64);
    let result = tuple.set(true);
    assert_eq!(result, Some(true));
    // Tuple should be unchanged
    assert_eq!(tuple.0, 42);
    assert_eq!(tuple.1, "hello");
    assert_eq!(tuple.2, 2.5);
}

#[test]
fn test_set_returns_value_when_duplicates() {
    let mut tuple = (42i32, "hello", 100i32);
    assert_eq!(tuple.set(200i32), Some(200));
    // Tuple should be unchanged
    assert_eq!(tuple.0, 42);
    assert_eq!(tuple.1, "hello");
    assert_eq!(tuple.2, 100);
}

#[test]
fn test_set_with_get() {
    let mut tuple = (42i32, "hello", 2.5f64);
    assert!(tuple.set(100i32).is_none());
    let updated_value: Option<&i32> = tuple.get();
    assert_eq!(updated_value, Some(&100));
}

#[test]
fn test_set_large_tuple() {
    let mut tuple = (1i32, 2u32, 3i64, 4u64, 5f32, 6f64, true, 'x');
    assert!(tuple.set(false).is_none());
    assert!(!tuple.6);
}
