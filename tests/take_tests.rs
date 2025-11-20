use tuple_set::TupleSet;

#[test]
fn test_take_option() {
    let mut tuple = (Some(42i32), "hello", Some(2.5f64));

    let value: Option<Option<i32>> = tuple.take();
    assert_eq!(value, Some(Some(42)));
    assert_eq!(tuple.0, None);

    let value: Option<Option<f64>> = tuple.take();
    assert_eq!(value, Some(Some(2.5)));
    assert_eq!(tuple.2, None);
}

#[test]
fn test_take_not_found() {
    let mut tuple = (Some(42i32), "hello", Some(2.5f64));

    let value: Option<bool> = tuple.take();
    assert_eq!(value, None);
}

#[test]
fn test_take_with_duplicates() {
    let mut tuple = (Some(42i32), "hello", Some(100i32));

    let value: Option<Option<i32>> = tuple.take();
    assert_eq!(value, None);
    // Tuple should be unchanged
    assert_eq!(tuple.0, Some(42));
    assert_eq!(tuple.2, Some(100));
}

#[test]
fn test_take_vec() {
    extern crate alloc;
    use alloc::{vec, vec::Vec};

    let mut tuple = (vec![1, 2, 3], "hello", 42i32);

    let value: Option<Vec<i32>> = tuple.take();
    assert_eq!(value, Some(vec![1, 2, 3]));
    assert_eq!(tuple.0, Vec::<i32>::default());
}

#[test]
fn test_take_single_element() {
    let mut tuple = (Some(999i32),);

    let value: Option<Option<i32>> = tuple.take();
    assert_eq!(value, Some(Some(999)));
    assert_eq!(tuple.0, None);
}
