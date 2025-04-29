#[cfg(test)]
mod tests {
    use data_struct_in_rust::data_structure::queue::Queue;

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new(3);
        assert!(queue.is_empty());

        assert!(queue.enqueue(1).is_ok());
        assert!(queue.enqueue(2).is_ok());
        assert!(queue.enqueue(3).is_ok());
        assert!(queue.enqueue(4).is_err()); // Should exceed capacity

        assert_eq!(queue.size(), 3);

        assert_eq!(queue.dequeue(), Some(1)); // FIFO: 1 should be dequeued first
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_size_limit() {
        let mut queue = Queue::new(2);
        assert!(queue.enqueue("a").is_ok());
        assert!(queue.enqueue("b").is_ok());
        assert!(queue.enqueue("c").is_err()); // Full

        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn test_is_empty_and_size() {
        let mut queue = Queue::new(1);
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);

        queue.enqueue(42).unwrap();
        assert!(!queue.is_empty());
        assert_eq!(queue.size(), 1);

        queue.dequeue();
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);
    }
}
