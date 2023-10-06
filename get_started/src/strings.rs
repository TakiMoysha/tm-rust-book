use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

pub fn link_string() {
    // base - String and &str
    let my_string: String = String::from("Hello");

    // heap
    let my_string2: &str = &my_string;

    //binary
    let my_string3: &str = "Hello";

    // stack
    let my_string4 = "Hello";

    // String - create/modify
    // &str - read/analyze

    let my_string5: &'static str = "Hello";
}

pub fn box_str() {
    let my_str: String = String::from("some text");
    let my_str_b: Box<str> = my_str.into_boxed_str();
    println!("{}", my_str_b);
}

pub fn reference_counter() {
    let some_text: &'static str = "some text";

    let txt_reference_1: Rc<str> = Rc::from(&some_text[5..]);

    let txt_reference_2 = Rc::clone(&txt_reference_1);
    let txt_reference_3 = Rc::clone(&txt_reference_1);
}

pub fn atomic_reference_counter() {
    let some_text: &'static str = "some text";
    let arc_ref: Arc<&&str> = Arc::from(&some_text);
}

pub fn cow_ref() {
    let some_text: &'static str = "it badword";

    fn sanitize(input: &str) -> Cow<str> {
        if input.contains("badword") {
            let sanitized: String = input.replace("badword", "****");
            return Cow::Owned(sanitized);
        }
        Cow::Borrowed(input)
    }

    let res = sanitize(&some_text);
    println!("{} -> {}", some_text, res);
}

pub fn main() {
    cow_ref();
}
