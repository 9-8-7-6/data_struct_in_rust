#[cfg(test)]
mod tests {
    use data_struct_in_rust::data_structure::vec::LVec;

    #[test]
    fn test_new_lvec() {
        let vec: LVec<i32> = LVec::new();
        assert_eq!(vec.len(), 0);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_push_and_len() {
        let mut vec = LVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_pop() {
        let mut vec = LVec::new();
        vec.push(10);
        vec.push(20);
        assert_eq!(vec.pop(), Some(20));
        assert_eq!(vec.pop(), Some(10));
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_insert() {
        let mut vec = LVec::new();
        vec.push(1);
        vec.push(3);
        vec.insert(1, 2);
        vec.insert(0, 0);
        vec.insert(10, 4);

        assert_eq!(vec.len(), 5);
        assert_eq!(vec.remove(0), Some(0));
        assert_eq!(vec.remove(0), Some(1));
        assert_eq!(vec.remove(0), Some(2));
        assert_eq!(vec.remove(0), Some(3));
        assert_eq!(vec.remove(0), Some(4));
    }

    #[test]
    fn test_remove() {
        let mut vec = LVec::new();
        vec.push(5);
        vec.push(6);
        vec.push(7);
        assert_eq!(vec.remove(1), Some(6));
        assert_eq!(vec.remove(1), Some(7));
        assert_eq!(vec.remove(0), Some(5));
        assert_eq!(vec.remove(0), None); // 空的情況
    }

    #[test]
    fn test_append() {
        let mut a = LVec::new();
        a.push(1);
        a.push(2);

        let mut b = LVec::new();
        b.push(3);
        b.push(4);

        a.append(&mut b);

        assert_eq!(a.len(), 4);
        assert_eq!(b.len(), 0); // 被清空了

        assert_eq!(a.remove(0), Some(1));
        assert_eq!(a.remove(0), Some(2));
        assert_eq!(a.remove(0), Some(3));
        assert_eq!(a.remove(0), Some(4));
    }
}
