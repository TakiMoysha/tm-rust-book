//! - Show stack alloc ring buffer using array allocated on stack.
//! - And pre-allocate using the pattern `internal_storage: [Option<T>; N]`.
//! - Show this constructor magic: `internal_storage: [(); N].map(|_| None)`.
//! - Show this generic header: `pub struct RingBuffer<T, const N: usize>`.
//! - Show the impl block with the same generic header: `impl<T, const N: usize>`.

pub struct RingBuffer<T, const N: usize> {
    internal_storage: [Option<T>; N],
    head: usize,
    tail: usize,
    count: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        Self {
            internal_storage: [(); N].map(|_| None),
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    pub fn len(&self) -> usize {
        self.count
    }
    pub fn cap(&self) -> usize {
        N
    }

    pub fn add(&mut self, item: T) {
        if self.count == N {
            // Buffer is full, overwrite the oldest item.
            self.tail = (self.tail + 1) % N;
        } else {
            self.count += 1;
        }
        self.internal_storage[self.head] = Some(item);
        self.head = (self.head + 1) % N;
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.count == 0 {
            return None; // Buffer is empty.
        }
        let item = self.internal_storage[self.tail].take();
        self.tail = (self.tail + 1) % N;
        self.count -= 1;
        item
    }
}

impl<T, const N: usize> Default for RingBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod ring_buffer_inline_tests {
    use super::*;

    /// Add to the tail of the queue and remove from the head of the queue.
    #[test]
    pub fn test_queue_api() {
        let mut rb = RingBuffer::<u8, 4>::new();

        // Partially fill the ring buffer.
        {
            rb.add(1); // Add to the tail of the queue.
            rb.add(2);
            rb.add(3);
            assert_eq!(rb.len(), 3);
            assert_eq!(rb.cap(), 4);

            let a = rb.remove(); // Remove from the head of the queue.
            let b = rb.remove();
            let c = rb.remove();

            assert_eq!(a, Some(1));
            assert_eq!(b, Some(2));
            assert_eq!(c, Some(3));
        }

        // Fill the ring buffer to capacity.
        {
            for i in 0..4 {
                rb.add(i);
            }
            assert_eq!(rb.remove(), Some(0));
            assert_eq!(rb.remove(), Some(1));
            assert_eq!(rb.remove(), Some(2));
            assert_eq!(rb.remove(), Some(3));
            assert_eq!(rb.remove(), None);
        }

        // Overfill the ring buffer.
        {
            rb.add(1);
            rb.add(2);
            rb.add(3);
            rb.add(4);
            rb.add(5);

            assert_eq!(rb.len(), 4);
            assert_eq!(rb.cap(), 4);

            assert_eq!(rb.remove(), Some(2));
            assert_eq!(rb.remove(), Some(3));
            assert_eq!(rb.remove(), Some(4));
            assert_eq!(rb.remove(), Some(5));
            assert_eq!(rb.remove(), None);
        }
    }
}
