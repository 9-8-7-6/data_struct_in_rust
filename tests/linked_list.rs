use data_struct_in_rust::db::linked_list::List;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut list = List::new();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);

        list.push(10);
        list.push(20);
        list.push(30); // Top -> 30 -> 20 -> 10

        assert_eq!(list.size(), 3);
        assert_eq!(list.pop(), Some(30));
        assert_eq!(list.pop(), Some(20));
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.pop(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        list.push(42);
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.peek_mut(), Some(&mut 42));
    }

    #[test]
    fn test_peek_mut() {
        let mut list = List::new();
        list.push(100);
        if let Some(val) = list.peek_mut() {
            *val = 200;
        }
        assert_eq!(list.peek(), Some(&200));
    }

    #[test]
    fn test_into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3); // Top -> 3 -> 2 -> 1

        let elems: Vec<_> = list.into_iter().collect();
        assert_eq!(elems, vec![3, 2, 1]);
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push("a");
        list.push("b");
        list.push("c"); // Top -> c -> b -> a

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&"c"));
        assert_eq!(iter.next(), Some(&"b"));
        assert_eq!(iter.next(), Some(&"a"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3); // Top -> 3 -> 2 -> 1

        for val in list.iter_mut() {
            *val *= 2;
        }

        let elems: Vec<_> = list.into_iter().collect();
        assert_eq!(elems, vec![6, 4, 2]);
    }
}
