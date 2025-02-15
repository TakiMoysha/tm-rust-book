mod base_iterators {
    use std::iter::{IntoIterator, Iterator};

    pub struct Counter {
        count: u32,
        limit: u32,
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            if self.count < self.limit {
                self.count += 1;
                return Some(self.count);
            }
            None
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn should_work_stdlib_iterator_trait() {
            let mut counter = Counter { count: 0, limit: 5 };
            assert_eq!(counter.next(), Some(1));
            assert_eq!(counter.next(), Some(2));
            assert_eq!(counter.next(), Some(3));
            assert_eq!(counter.next(), Some(4));
            assert_eq!(counter.next(), Some(5));
            assert_eq!(counter.next(), None);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
