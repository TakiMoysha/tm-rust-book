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

    // demo type bounds
    #[allow(dead_code)]
    fn consumer_safe_iterator<I>(iter: I)
    where
        I: IntoIterator<Item: Send> + Send,
    {
    }

    #[cfg(test)]
    mod tests {
        use std::sync::mpsc::{self, Receiver, Sender};
        use std::{thread, time::Duration};

        use super::*;

        #[test]
        fn should_iterate_in_parallel() {
            let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

            let mut counter = SafeCounter::new(5);
            let mut counter_clone = counter.clone();

            thread::scope(|s| {
                s.spawn(move || {
                    assert_eq!(counter.next(), Some(1));
                    tx.send(0).unwrap();
                    thread::sleep(Duration::from_millis(100));
                    assert_eq!(counter.next(), Some(3));
                });
                s.spawn(move || {
                    rx.recv().unwrap();
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

mod double_ended_iterator {
    use std::iter::DoubleEndedIterator;

    pub struct Counter {
        count: usize,
        limit: usize,
    }

    impl Iterator for Counter {
        type Item = usize;

        fn next(&mut self) -> Option<usize> {
            if self.count < self.limit {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    impl DoubleEndedIterator for Counter {
        fn next_back(&mut self) -> Option<usize> {
            todo!("not implemented, iterate to start")
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_iterate_to_start() {
            let mut counter = Counter { count: 0, limit: 5 };
            counter.map(|el| println!("{}", el)).count();
            // assert_eq!(counter.next_back(), None);
            // assert_eq!(counter.next_back(), Some(4));
            // assert_eq!(counter.next_back(), Some(3));
            // assert_eq!(counter.next_back(), Some(2));
            // assert_eq!(counter.next_back(), Some(1));
            // assert_eq!(counter.next_back(), None);
        }
    }
}

mod seek_iterators {
    use std::io::{self, Read, Seek, SeekFrom};

    struct SeekIterator {
        data: Vec<u8>,
        position: usize,
    }

    impl SeekIterator {
        fn new(data: Vec<u8>) -> Self {
            SeekIterator { data, position: 0 }
        }
    }

    impl Read for SeekIterator {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if self.position >= self.data.len() {
                return Ok(0); // Конец данных
            }

            let bytes_read = std::cmp::min(buf.len(), self.data.len() - self.position);
            buf[..bytes_read]
                .copy_from_slice(&self.data[self.position..self.position + bytes_read]);
            self.position += bytes_read;

            Ok(bytes_read)
        }
    }

    impl Seek for SeekIterator {
        fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
            match pos {
                SeekFrom::Start(offset) => {
                    if offset > self.data.len() as u64 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "Seek out of bounds",
                        ));
                    }
                    self.position = offset as usize;
                }
                SeekFrom::Current(offset) => {
                    let new_position = self.position as i64 + offset;
                    if new_position < 0 || new_position > self.data.len() as i64 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "Seek out of bounds",
                        ));
                    }
                    self.position = new_position as usize;
                }
                SeekFrom::End(offset) => {
                    let new_position = self.data.len() as i64 + offset;
                    if new_position < 0 || new_position > self.data.len() as i64 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "Seek out of bounds",
                        ));
                    }
                    self.position = new_position as usize;
                }
            }

            Ok(self.position as u64)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_seek_correct() {
            let data = b"Hello, world!".to_vec();
            let mut iterator = SeekIterator::new(data);

            let mut buffer = [0; 5];

            iterator.read_exact(&mut buffer).unwrap();
            println!("Read: {:?}", String::from_utf8_lossy(&buffer));

            iterator.seek(SeekFrom::Start(7)).unwrap();

            iterator.read_exact(&mut buffer).unwrap();
            println!("Read: {:?}", String::from_utf8_lossy(&buffer));
        }
    }
}

mod compile_time_iterators {
    // pub const trait IntoIterator {
    //     type Item;
    //     type IntoIter: const Iterator<Item = Self::Item>;
    //     const fn into_iter(self) -> Self::IntoIter;
    // }

    // pub const trait Iterator {
    //     type Item;
    //     const fn next(&mut self) -> Option<Self::Item>;
    //     const fn size_hint(&self) -> (usize, Option<usize>) { .. }
    // }

    // !WARN: idk what write this
}

mod lending_iterators {

    struct LeadingIterator<'a, T> {
        iter: &'a [T],
        index: usize,
    }

    impl<'a, T> LeadingIterator<'a, T> {
        fn new(iter: &'a [T]) -> Self {
            LeadingIterator { iter, index: 0 }
        }
    }

    impl<'a, T> Iterator for LeadingIterator<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.iter.len() {
                let item = &self.iter[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_return_reference() {
            let data = [1, 2, 3, 4, 5];
            let mut iter = LeadingIterator::new(&data);
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&4));
            assert_eq!(iter.next(), Some(&5));
            assert_eq!(iter.next(), None);
        }
    }
}

mod with_return_value_iterator {
    use std::ops::ControlFlow;

    struct ReturningCounter {
        index: usize,
        limit: usize,
    }

    impl ReturningCounter {
        fn new(limit: usize) -> Self {
            ReturningCounter { index: 0, limit }
        }
    }

    impl Iterator for ReturningCounter {
        type Item = ControlFlow<u32, usize>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.limit {
                let value = self.index;
                self.index += 1;
                Some(ControlFlow::Continue(value))
            } else {
                Some(ControlFlow::Break(self.index as u32))
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_return_correct_break_value() {
            let mut iter = ReturningCounter::new(5);

            for res in iter {
                match res {
                    ControlFlow::Continue(value) => println!("Yielded: {}", value),
                    ControlFlow::Break(value) => {
                        println!("Finished with: {}", value);
                        assert_eq!(value, 5);
                        break;
                    }
                }
            }
        }
    }
}

mod with_next_argument_iterator {
    struct ArgumentCounter {
        index: usize,
        limit: usize,
    }

    impl ArgumentCounter {
        fn new(limit: usize) -> Self {
            Self { index: 0, limit }
        }
    }

    trait ArgumentIterator {
        type Item;
        type Args;
        fn next(&mut self, args: Self::Args) -> Option<Self::Item>;
    }

    impl ArgumentIterator for ArgumentCounter {
        type Item = usize;
        type Args = usize;

        fn next(&mut self, increment: Self::Item) -> Option<Self::Item> {
            if self.index < self.limit {
                self.index += increment;
                Some(self.index)
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_work() {
            let mut iter = ArgumentCounter::new(5);
            assert_eq!(iter.next(1), Some(1));
            assert_eq!(iter.next(1), Some(2));
            assert_eq!(iter.next(2), Some(4));
            assert_eq!(iter.next(3), Some(7));
            assert_eq!(iter.next(0), None);
        }
    }
}

mod short_circuiting_iterator {
    struct ShortCircuitingCounter<I> {
        inner: I,
        limit: usize,
    }

    impl<I: Iterator<Item = usize>> ShortCircuitingCounter<I> {
        fn new(inner: I, limit: usize) -> Self {
            Self { inner, limit }
        }
    }

    impl<I: Iterator<Item = usize>> Iterator for ShortCircuitingCounter<I> {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            let value = self.inner.next()?;

            if value < self.limit {
                Some(value)
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_work() {
            // ?NOTE: iterator have size 10, but stop condition is 5
            let mut iter = ShortCircuitingCounter::new(0..10, 3);
            assert_eq!(iter.next(), Some(0));
            assert_eq!(iter.next(), Some(1));
            assert_eq!(iter.next(), Some(2));
            assert_eq!(iter.next(), None);
        }
    }
}

mod address_sessitive_iterator {
    use std::marker::PhantomData;
    use std::pin::Pin;

    pub trait AddressSensitiveIterator {
        type Item;
        fn next(self: Pin<&mut Self>) -> Option<Self::Item>;
    }

    struct AddressSensitiveCounter {
        data: Vec<i32>,
        index: usize,
    }

    impl AddressSensitiveCounter {
        fn new(data: Vec<i32>) -> Self {
            Self { data, index: 0 }
        }
    }

    impl Iterator for AddressSensitiveCounter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }
    // impl AddressSensitiveIterator for AddressSensitiveCounter {
    //     type Item = i32;
    //
    //     fn next(self: Pin<&mut Self>) -> Option<Self::Item> {
    //         let this = self.get_mut();
    //
    //         if this.index < this.data.len() {
    //             let value = this.data[this.index];
    //             this.index += 1;
    //             Some(value)
    //         } else {
    //             None
    //         }
    //     }
    // }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_return_pin_ref_to_data() {
            let data = [0, 1, 2];
            let mut iter = AddressSensitiveCounter::new(data.to_vec());
            // ?NOTE: for pin address
            let pinned_iter = Pin::new(&mut iter);

            for (index, value) in pinned_iter.get_mut().enumerate() {
                assert_eq!(value, index as i32);
            }
        }

        #[test]
        fn should_save_address() {
            let data = [0, 1, 2];
            let mut iter = AddressSensitiveCounter::new(data.to_vec());
            let pinned_iter = Pin::new(&mut iter);
            let address_before = &*pinned_iter as *const _ as usize;
            let new_address_iter = pinned_iter;
            assert_eq!(address_before, &*new_address_iter as *const _ as usize);
        }

        #[test]
        fn should_change_address() {
            let data = [0, 1, 2];
            let iter = AddressSensitiveCounter::new(data.to_vec());
            let address_before = &iter as *const _ as usize;
            let moved_iter = iter;
            assert_ne!(address_before, &moved_iter as *const _ as usize);
        }
    }
}

mod iterator_guaranteeing_destruct {
    #[derive(Clone)]
    struct Resource {
        id: String,
    }

    impl Drop for Resource {
        fn drop(&mut self) {
            println!("Dropping {}", self.id);
        }
    }

    struct ResourceIterator<'a> {
        resources: &'a Vec<Resource>,
        index: usize,
    }

    impl<'a> ResourceIterator<'a> {
        fn new(resources: &'a Vec<Resource>) -> Self {
            ResourceIterator {
                resources,
                index: 0,
            }
        }
    }

    impl<'a> Iterator for ResourceIterator<'a> {
        type Item = &'a Resource;

        // ?NOTE: return reference to resource
        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.resources.len() {
                let val = &self.resources[self.index];
                self.index += 1;
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
        fn should_destruct_regularly_case() {
            let resource = [
                Resource {
                    id: "a".to_string(),
                },
                Resource {
                    id: "b".to_string(),
                },
            ]
            .to_vec();
            {
                let mut iter = ResourceIterator::new(&resource);
                assert_eq!(dbg!(&iter.next().unwrap().id), "a");
            }
        }
    }
}

mod async_iterator {
    // !TODO: added async runtime
}

mod concurrent_iterator {

    #[cfg(test)]
    mod tests {
        use super::*;
        use rayon::prelude::*;

        #[test]
        fn test_parallel_iterator() {
            let nums: Vec<i32> = (0..100).collect();
            let squared: Vec<i32> = nums.par_iter().map(|x| x * x).collect();
            println!("{:?}", squared);
        }
    }
}

fn main() {}
