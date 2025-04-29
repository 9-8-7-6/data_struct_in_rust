#[cfg(test)]
mod tests {
    use data_struct_in_rust::data_structure::deque::Deque;
    #[test]
    fn test_push_front_back_and_pop() {
        let mut deque = Deque::new(3);
        assert!(deque.is_empty());

        assert!(deque.push_back(1).is_ok()); // [1]
        assert!(deque.push_front(2).is_ok()); // [2, 1]
        assert!(deque.push_back(3).is_ok()); // [2, 1, 3]
        assert!(deque.push_back(4).is_err()); // Exceeds capacity

        assert_eq!(deque.size(), 3);

        assert_eq!(deque.pop_front(), Some(2)); // [1, 3]
        assert_eq!(deque.pop_back(), Some(3)); // [1]
        assert_eq!(deque.pop_back(), Some(1)); // []
        assert_eq!(deque.pop_back(), None);
        assert!(deque.is_empty());
    }

    #[test]
    fn test_front_and_back_push_pop_sequence() {
        let mut deque = Deque::new(4);

        deque.push_front("a").unwrap(); // ["a"]
        deque.push_back("b").unwrap(); // ["a", "b"]
        deque.push_front("c").unwrap(); // ["c", "a", "b"]
        deque.push_back("d").unwrap(); // ["c", "a", "b", "d"]

        assert_eq!(deque.size(), 4);
        assert!(deque.push_back("e").is_err());

        assert_eq!(deque.pop_front(), Some("c"));
        assert_eq!(deque.pop_back(), Some("d"));
        assert_eq!(deque.pop_front(), Some("a"));
        assert_eq!(deque.pop_back(), Some("b"));
        assert!(deque.is_empty());
    }

    #[test]
    fn test_empty_pop() {
        let mut deque = Deque::<i32>::new(1);
        assert_eq!(deque.pop_front(), None);
        assert_eq!(deque.pop_back(), None);
    }
}
