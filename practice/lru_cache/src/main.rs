mod double_linked_list {
    struct Node<T> {
        prev: Option<Box<Node<T>>>,
        next: Option<Box<Node<T>>>,
        data: u64,
    }
}

fn main() {
    println!("Hello, world!");
}
