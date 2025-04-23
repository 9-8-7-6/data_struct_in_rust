#[cfg(test)]
mod tests {
    use data_struct_in_rust::db::vec::LVec;

    #[test]
    fn test_new_lvec() {
        let LVec: LVec<i32> = LVec::new();
        assert_eq!(LVec.len(), 0);
        assert!(LVec.is_empty());
    }

    #[test]
    fn test_push_and_len() {
        let mut LVec = LVec::new();
        LVec.push(1);
        LVec.push(2);
        LVec.push(3);
        assert_eq!(LVec.len(), 3);
    }

    #[test]
    fn test_pop() {
        let mut LVec = LVec::new();
        LVec.push(10);
        LVec.push(20);
        assert_eq!(LVec.pop(), Some(20));
        assert_eq!(LVec.pop(), Some(10));
        assert_eq!(LVec.pop(), None);
    }

    #[test]
    fn test_insert() {
        let mut LVec = LVec::new();
        LVec.push(1);
        LVec.push(3);
        LVec.insert(1, 2); // 插入到中間
        LVec.insert(0, 0); // 插入到前面
        LVec.insert(10, 4); // 超出範圍，插入末尾

        assert_eq!(LVec.len(), 5);
        assert_eq!(LVec.remove(0), Some(0));
        assert_eq!(LVec.remove(0), Some(1));
        assert_eq!(LVec.remove(0), Some(2));
        assert_eq!(LVec.remove(0), Some(3));
        assert_eq!(LVec.remove(0), Some(4));
    }

    #[test]
    fn test_remove() {
        let mut LVec = LVec::new();
        LVec.push(5);
        LVec.push(6);
        LVec.push(7);
        assert_eq!(LVec.remove(1), Some(6));
        assert_eq!(LVec.remove(1), Some(7));
        assert_eq!(LVec.remove(0), Some(5));
        assert_eq!(LVec.remove(0), None); // 空的情況
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
