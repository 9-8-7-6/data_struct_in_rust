#[cfg(test)]
mod tests {
    use data_struct_in_rust::data_structure::binary_tree::BinaryTree;

    #[test]
    fn test_create_node() {
        let tree = BinaryTree::new(10);
        assert_eq!(tree.get_key(), 10);
        assert!(tree.get_left().is_none());
        assert!(tree.get_right().is_none());
    }

    #[test]
    fn test_insert_left() {
        let mut tree = BinaryTree::new(10);
        tree.insert_left_tree(5);
        assert_eq!(tree.get_left().unwrap().key, 5);
    }

    #[test]
    fn test_insert_right() {
        let mut tree = BinaryTree::new(10);
        tree.insert_right_tree(20);
        assert_eq!(tree.get_right().unwrap().key, 20);
    }

    #[test]
    fn test_insert_left_stacking() {
        let mut tree = BinaryTree::new(10);
        tree.insert_left_tree(5);
        tree.insert_left_tree(2);
        let first = tree.get_left().unwrap();
        let second = first.left.as_ref().unwrap();
        assert_eq!(first.key, 2);
        assert_eq!(second.key, 5);
    }

    #[test]
    fn test_insert_right_stacking() {
        let mut tree = BinaryTree::new(10);
        tree.insert_right_tree(15);
        tree.insert_right_tree(25);
        let first = tree.get_right().unwrap();
        let second = first.right.as_ref().unwrap();
        assert_eq!(first.key, 25);
        assert_eq!(second.key, 15);
    }

    #[test]
    fn test_set_key() {
        let mut tree = BinaryTree::new("old");
        tree.set_key("new");
        assert_eq!(tree.get_key(), "new");
    }
}
