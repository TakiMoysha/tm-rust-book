mod base_iterators {
    use std::iter::Iterator;

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

mod fused_iterators {
    use std::iter::Iterator;

    pub struct EvenCounter {
        count: u32,
        limit: u32,
    }

    impl Iterator for EvenCounter {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            let val = self.count;
            self.count += 1;

            if val % 2 == 0 {
                Some(val)
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_work_like_standard_iterator_trait() {
            let mut counter = EvenCounter { count: 0, limit: 5 };
            assert_eq!(counter.next(), Some(0));
            assert_eq!(counter.next(), None);
            assert_eq!(counter.next(), Some(2));
            assert_eq!(counter.next(), None);
            assert_eq!(counter.next(), Some(4));
            assert_eq!(counter.next(), None);
        }

        #[test]
        fn should_return_none_after_fuse() {
            let mut counter = EvenCounter { count: 0, limit: 5 };
            assert_eq!(counter.next(), Some(0));
            assert_eq!(counter.next(), None);
            assert_eq!(counter.next(), Some(2));
            assert_eq!(counter.next(), None);
            let mut counter = counter.fuse();
            assert_eq!(counter.next(), Some(4));
            assert_eq!(counter.next(), None);
            assert_eq!(counter.next(), None);
            assert_eq!(counter.next(), None);
        }
    }
}

mod thread_safe_iterator {
    use std::{
        iter::Iterator,
        sync::{Arc, Mutex},
    };

    #[derive(Clone)]
    pub struct SafeCounter {
        count: Arc<Mutex<usize>>,
        limit: usize,
    }

    impl SafeCounter {
        fn new(limit: usize) -> Self {
            Self {
                count: Arc::new(Mutex::new(0)),
                limit,
            }
        }
    }

    impl Iterator for SafeCounter {
        type Item = usize;

        fn next(&mut self) -> Option<usize> {
            let mut count = self.count.lock().unwrap();
            if *count < self.limit {
                *count += 1;
                Some(*count)
            } else {
                None
            }
        }
    }

    unsafe impl Send for SafeCounter {}
    unsafe impl Sync for SafeCounter {}

    fn consumer_safe_iterator<I>(iter: I)
    where
        I: IntoIterator<Item: Send> + Send,
    {
    }

    #[cfg(test)]
    mod tests {
        use std::{thread, time::Duration};

        use super::*;

        #[test]
        fn should_work_stdlib_iterator_trait() {
            let mut counter = SafeCounter::new(5);
            let mut counter_clone = counter.clone();

            thread::scope(|s| {
                s.spawn(move || {
                    assert_eq!(counter.next(), Some(1));
                    thread::sleep(Duration::from_millis(500));
                    assert_eq!(counter.next(), Some(3));
                });
                s.spawn(move || {
                    thread::sleep(Duration::from_millis(400));
                    assert_eq!(counter_clone.next(), Some(2));
                });
            });
        }
    }
}

mod dyn_compatible_iterator {
    use std::iter::Iterator;

    trait Entity {}

    #[derive(Clone)]
    struct Animal(String, u32);
    impl Entity for &Animal {}
    #[derive(Clone)]
    struct Creature(String);
    impl Entity for Creature {}

    pub struct Zoo {
        animals: Vec<String>,
        entity: Box<dyn Entity>,
    }

    impl Zoo {
        // 'static lifetime indicated, what Entity type lifes more than Zoo
        // Box puts the object in a heap, because The size is not determined
        //
        // ?NOTE: take any type that implements Entity trait, with known scope at compile time
        // the method will be created for a specific type
        fn from_vec(animals: Vec<&str>, entity: impl Entity + 'static) -> Self {
            let animals = animals.iter().map(|s| s.to_string()).collect();
            Zoo {
                animals,
                entity: Box::new(entity),
            }
        }

        // ?NOTE: accepts the pointer to a dynamically dedicated object
        // `dyn` keyword allows skip the type difinition at compile time
        // at runtime will be dynamically dispatched
        // that more flexible than `impl Entity + 'static`
        fn from_vec_dyn(animals: Vec<&str>, entity: Box<dyn Entity>) -> Self {
            let animals = animals.iter().map(|s| s.to_string()).collect();
            Zoo { animals, entity }
        }
    }

    impl Iterator for Zoo {
        type Item = String;

        fn next(&mut self) -> Option<String> {
            self.animals.pop()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const ANIMALS: [&str; 3] = ["dog", "cat", "mouse"];

        #[test]
        fn should_work_dyn_iterator_work() {
            let entity = Creature("creature".to_string());
            let mut zoo = Zoo::from_vec(ANIMALS.to_vec(), entity);
            assert_eq!(zoo.next(), Some("mouse".to_string()));
            assert_eq!(zoo.next(), Some("cat".to_string()));
            assert_eq!(zoo.next(), Some("dog".to_string()));
            assert_eq!(zoo.next(), None);
        }

        #[test]
        fn should_work_iterator_with_compiled_type() {
            let zoo = Zoo::from_vec(ANIMALS.to_vec(), Creature("creature".to_string()));
            let dyn_zoo = zoo;
        }

        #[test]
        fn should_work_iter_with_dynamic_dispatch() {
            let entity = Creature("creature".to_string());
            let mut zoo = Zoo::from_vec_dyn(ANIMALS.to_vec(), Box::new(entity));
            assert_eq!(zoo.next(), Some("mouse".to_string()));
            assert_eq!(zoo.next(), Some("cat".to_string()));
            assert_eq!(zoo.next(), Some("dog".to_string()));
            assert_eq!(zoo.next(), None);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
