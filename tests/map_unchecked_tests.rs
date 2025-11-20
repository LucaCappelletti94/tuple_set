use tuple_set::TupleSet;

#[test]
fn test_map_unchecked_simple() {
    let mut tuple = (42i32, "hello", 2.5f64);
    let result = unsafe {
        tuple.map_unchecked::<i32, _, _>(|x: &mut i32| {
            *x *= 2;
            *x
        })
    };
    assert_eq!(result, 84);
    assert_eq!(tuple.0, 84);
}

#[test]
#[should_panic(expected = "not found")]
fn test_map_unchecked_panic_not_found() {
    let mut tuple = (42i32, "hello", 2.5f64);
    unsafe {
        tuple.map_unchecked::<bool, _, _>(|x: &mut bool| *x = true);
    }
}

#[test]
fn test_map_unchecked_with_duplicates() {
    let mut tuple = (10i32, 20i32, "test");
    let result = unsafe {
        tuple.map_unchecked::<i32, _, _>(|x: &mut i32| {
            *x += 1;
            *x
        })
    };
    // Should modify the first occurrence
    assert_eq!(result, 11);
    assert_eq!(tuple.0, 11);
    assert_eq!(tuple.1, 20);
}

#[test]
fn test_map_unchecked_single_element() {
    let mut tuple = (999i32,);
    let result = unsafe {
        tuple.map_unchecked::<i32, _, _>(|x: &mut i32| {
            *x += 1;
            *x
        })
    };
    assert_eq!(result, 1000);
    assert_eq!(tuple.0, 1000);
}

#[test]
fn test_map_unchecked_large_tuple() {
    let mut tuple = (1i32, 2u32, 3i64, 4u64, 5f32, 6f64, false, 'x');
    let result = unsafe {
        tuple.map_unchecked::<bool, _, _>(|b: &mut bool| {
            *b = true;
            *b
        })
    };
    assert!(result);
    assert!(tuple.6);
}
