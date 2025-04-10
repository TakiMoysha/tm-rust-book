use std::fmt::Display;
use std::mem;
use std::ops::Deref;

//  =========================================== pointer to value
#[derive(Debug)]
pub struct BoxValue<T>(T);

impl<T> BoxValue<T> {
    fn new(v: T) -> BoxValue<T> {
        BoxValue(v)
    }
}

impl<T> Deref for BoxValue<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

//  ===========================================
#[derive(Debug)]
pub struct LinkedList {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct Node {
    value: i32,
    next: Link,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: Link::Empty }
    }

    pub fn push(&mut self, value: i32) {
        let node = Box::new(Node {
            value,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut node) = cur_link {
            cur_link = mem::replace(&mut node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_created_simple_list() {
        let mut list = LinkedList::new();
        list.push(2);
        list.pop();
        list.push(4);
        list.push(5);
    }
}

