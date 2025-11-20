use tuple_set::TupleSet;

#[test]
fn test_count_single_occurrence() {
    let tuple = (42i32, "hello", 2.5f64);
    assert_eq!(tuple.count::<i32>(), 1);
    assert_eq!(tuple.count::<&str>(), 1);
    assert_eq!(tuple.count::<f64>(), 1);
}

#[test]
fn test_count_not_found() {
    let tuple = (42i32, "hello", 2.5f64);
    assert_eq!(tuple.count::<bool>(), 0);
    assert_eq!(tuple.count::<char>(), 0);
}

#[test]
fn test_count_duplicates() {
    let tuple = (42i32, "hello", 100i32);
    assert_eq!(tuple.count::<i32>(), 2);
}

#[test]
fn test_count_many_duplicates() {
    let tuple = (1i32, 2i32, 3i32, 4i32, 5i32);
    assert_eq!(tuple.count::<i32>(), 5);
}

#[test]
fn test_contains_unique_true() {
    let tuple = (42i32, "hello", 2.5f64);
    assert!(tuple.contains_unique::<i32>());
    assert!(tuple.contains_unique::<&str>());
    assert!(tuple.contains_unique::<f64>());
}

#[test]
fn test_contains_unique_false_not_found() {
    let tuple = (42i32, "hello", 2.5f64);
    assert!(!tuple.contains_unique::<bool>());
}

#[test]
fn test_contains_unique_false_duplicates() {
    let tuple = (42i32, "hello", 100i32);
    assert!(!tuple.contains_unique::<i32>());
}
