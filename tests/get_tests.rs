use tuple_set::TupleSet;

#[test]
fn test_get_single_element() {
    let tuple = (42i32,);
    let value: Option<&i32> = tuple.get();
    assert_eq!(value, Some(&42));
}

#[test]
fn test_get_from_tuple() {
    let tuple = (42i32, "hello", 2.5f64);
    let value: Option<&i32> = tuple.get();
    assert_eq!(value, Some(&42));

    let value: Option<&&str> = tuple.get();
    assert_eq!(value, Some(&"hello"));

    let value: Option<&f64> = tuple.get();
    assert_eq!(value, Some(&2.5));
}

#[test]
fn test_get_not_found() {
    let tuple = (42i32, "hello", 2.5f64);
    let value: Option<&bool> = tuple.get();
    assert_eq!(value, None);
}

#[test]
fn test_get_with_duplicates() {
    let tuple = (42i32, "hello", 100i32);
    let value: Option<&i32> = tuple.get();
    assert_eq!(value, None);
}

#[test]
fn test_get_large_tuple() {
    let tuple = (1i32, 2u32, 3i64, 4u64, 5f32, 6f64, true, 'x');
    let value: Option<&bool> = tuple.get();
    assert_eq!(value, Some(&true));

    let value: Option<&char> = tuple.get();
    assert_eq!(value, Some(&'x'));
}
