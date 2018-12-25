use crate::*;

// List used for calling from_list tests.
fn get_list<'a>() -> &'a [(i16, i16); 8] {
    &[
        (0, 0),         // Default test,
        (100, 100),     // Two positive values test,
        (50, -50),      // One positive one negative test.
        (-9999, 9999),  // Larger values test.
        (0, 1),         // Close to zero test.
        (-1, -1),       // Close to zero double negative test.
        (0, 0),         // Duplicate of default test.
        (-9990, -9999), // Two large negative numbers test.
    ]
}

#[test]
fn coordinate() {
    let result = Coordinate::from_list(get_list());
    assert_eq!(result.len(), 8);
}

#[test]
fn node() {
    let result = Node::from_list(get_list());
    assert_eq!(result.len(), 8);
}

#[test]
fn group() {
    let result = Group::from_list(get_list());
    assert_eq!(result.len(), 8);
}
