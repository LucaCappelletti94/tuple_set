use tuple_set::TupleSet;

#[test]
fn test_map_simple() {
    let mut tuple = (42i32, "hello", 2.5f64);
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
    let result = tuple.map(|c: &mut char| {
        let old = *c;
        *c = 'b';
        old
    });
    assert_eq!(result, Some('a'));
    assert_eq!(tuple.1, 'b');
}

#[test]
fn test_map_returns_none_when_not_found() {
    let mut tuple = (42i32, "hello", 2.5f64);
    assert_eq!(tuple.map(|_x: &mut bool| true), None);
}

#[test]
fn test_map_returns_none_when_duplicates() {
    let mut tuple = (42i32, 100i32, "hello");
    assert_eq!(tuple.map(|_x: &mut i32| 200), None);
}

#[test]
fn test_map_single_element() {
    let mut tuple = (42i32,);
    let result = tuple.map(|x: &mut i32| {
        *x += 10;
        *x
    });
    assert_eq!(result, Some(52));
    assert_eq!(tuple.0, 52);
}

#[test]
fn test_map_large_tuple() {
    let mut tuple = (1i32, 2u32, 3i64, 4u64, 5f32, 6f64, false, 'x');
    let result = tuple.map(|b: &mut bool| {
        *b = true;
        *b
    });
    assert_eq!(result, Some(true));
    assert!(tuple.6);
}

#[test]
fn test_map_preserves_other_values() {
    let mut tuple = (42i32, "hello", 2.5f64);
    tuple.map(|x: &mut i32| {
        *x = 999;
        *x
    });
    assert_eq!(tuple.0, 999);
    assert_eq!(tuple.1, "hello");
    assert_eq!(tuple.2, 2.5);
}
