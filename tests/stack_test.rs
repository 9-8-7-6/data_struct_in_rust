use data_struct_in_rust::data_structure::stack::Stack;

#[test]
fn test_stack_peek() {
    let mut s = Stack::new();
    s.push(99);
    assert_eq!(s.top(), Some(&99));
    assert_eq!(s.len(), 1);
    assert_eq!(s.pop(), Some(99));
    assert_eq!(s.is_empty(), true);
}
