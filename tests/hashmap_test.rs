#[cfg(test)]
mod tests {
    use data_struct_in_rust::db::hashmap::Hashmap;
    #[test]
    fn test_insert_and_get() {
        let mut map = Hashmap::<i32>::new(11);
        map.insert(1, 10);
        map.insert(12, 20);
        map.insert(23, 30);

        assert_eq!(map.get(1), Some(&10));
        assert_eq!(map.get(12), Some(&20));
        assert_eq!(map.get(23), Some(&30));
    }

    #[test]
    fn test_remove() {
        let mut map = Hashmap::<i32>::new(11);
        map.insert(5, 50);
        assert_eq!(map.get(5), Some(&50));

        let removed = map.remove(5);
        assert_eq!(removed, Some(50));
        assert_eq!(map.get(5), None);
    }

    #[test]
    fn test_contains() {
        let mut map = Hashmap::<i32>::new(11);
        map.insert(8, 80);
        assert!(map.contains(8));
        assert!(!map.contains(9));
    }

    #[test]
    fn test_len() {
        let mut map = Hashmap::<i32>::new(11);
        assert_eq!(map.len(), 0);
        map.insert(3, 30);
        map.insert(14, 40);
        assert_eq!(map.len(), 2);

        map.remove(3);
        assert_eq!(map.len(), 1);
    }

    #[test]
    #[should_panic(expected = "key must > 0")]
    fn test_insert_zero_key_should_panic() {
        let mut map = Hashmap::<i32>::new(11);
        map.insert(0, 100);
    }

    #[test]
    #[should_panic(expected = "key must > 0")]
    fn test_get_zero_key_should_panic() {
        let map = Hashmap::<i32>::new(11);
        map.get(0);
    }

    #[test]
    fn test_overflow_insertion() {
        let mut map = Hashmap::<i32>::new(3);
        map.insert(1, 100);
        map.insert(4, 200);
        map.insert(7, 300);
        map.insert(10, 400);
        assert_eq!(map.len(), 3);
    }
}
