pub fn _offtop_1_palindrom() {
    fn _is_palindrom(text: String) -> bool {
        let half = text.len() / 2;
        let forward = text.bytes().take(half);
        let backward = text.bytes().rev().take(half);
        forward.eq(backward)
    }

    assert_eq!(_is_palindrom(String::from("aboba")), true);
    assert_eq!(_is_palindrom(String::from("abaaba")), true);
    assert_eq!(_is_palindrom(String::from("abaaasba")), false);
}

pub fn _offtop_2_links() {
    use std::ops::Deref; // позволяет умному указателю вести себя как ссылка
    use std::ops::Drop; // событие выхода из области видимости экземпляра

    // Box<T> - размещает значение в куче
    // Rc<T> - подсчет ссылок

    // RefMut<T> and Ref<T> - доступ через RefCell<T> - применяет правила заимствования

    fn _box_example() {
        // enum List {
        //     Cons(i32, Box),
        //     Nil,
        // }
        // use List::{ Cons, Nil };
        // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    _box_example();

    fn _rc_example() {
        // use std::rc::Rc;
        // enum List {
        //     Cons(i32, Rc<i32>),
        //     Nil,
        // }
        // use List::{ Cons, Nil };

        // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        // println!("count after createing a = {}", Rc::strong_count(&a));
        // let b = Cons(3, Rc::clone(&a));
        // println!("count after creating b = {}", Rc::strong_count(&a));
        // {
        //     let c = Cons(4, Rc::clone(&a));
        //     println!("count after creating c = {}", Rc::strong_count(&a));
        // }
        // println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
    _rc_example();

    fn _refcell_example() {
        // #[derive(Debug)]
        // enum List {
        //     Cons(Rc<RefCell>, Rc),
        //     Nil,
        // }

        // use List::{Cons, Nil};
        // use std::rc::Rc;
        // use std:: cell::RefCell;

        // let value = Rc::new(RefCell::new(5));
        // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        // *value.borrow_mut() += 10;
        // println!("a after = {:?}", a);
    }
    _refcell_example();
}
