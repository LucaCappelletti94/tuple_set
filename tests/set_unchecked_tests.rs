use tuple_set::TupleSet;

#[test]
fn test_set_unchecked_simple() {
    let mut tuple = (42i32, "hello", 2.5f64);
    unsafe {
        tuple.set_unchecked(100i32);
    }
    assert_eq!(tuple.0, 100);
}

#[test]
#[should_panic(expected = "Type")]
fn test_set_unchecked_panic_not_found() {
    let mut tuple = (42i32, "hello", 2.5f64);
    unsafe {
        tuple.set_unchecked(true);
    }
}

#[test]
fn test_set_unchecked_with_duplicates() {
    let mut tuple = (42i32, 100i32, "hello");
    unsafe {
        tuple.set_unchecked(82i32);
    }
    // Should set the first occurrence
    assert_eq!(tuple.0, 82);
    assert_eq!(tuple.1, 100);
}

#[test]
fn test_set_unchecked_single_element() {
    let mut tuple = (42i32,);
    unsafe {
        tuple.set_unchecked(999i32);
    }
    assert_eq!(tuple.0, 999);
}

#[test]
fn test_set_unchecked_large_tuple() {
    let mut tuple = (1i32, 2u32, 3i64, 4u64, 5f32, 6f64, true, 'x');
    unsafe {
        tuple.set_unchecked('z');
    }
    assert_eq!(tuple.7, 'z');
}
