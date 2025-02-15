mod linked_list {
    type Payload = u64;

    pub struct Node {
        data: Box<Payload>,
        next: Option<Box<Node>>,
    }

    macro_rules! create_linked_list {
        ($($x:expr),*) => {
            {
                let mut head = None;
                $(
                    let node = Node::new($x);
                    node.next = head;
                    head = Some(node);
                )*
                head
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_create_simple_list() {}

        #[test]
        fn should_create_simple_list_by_macros() {}
    }
}

mod double_linked_list {
    type Payload = u64;
}

fn main() {
    println!("Hello, world!");
}
