#[warn(dead_code)]
fn _let_1() {
    let (a, b, c, d, e): (i8, i16, i32, i64, i128) =
        (i8::MAX, i16::MAX, i32::MAX, i64::MAX, i128::MAX);
    println!("i8\ti16\ti32\t\ti64\t\t\ti128");
    println!("{a}\t{b}\t{c}\t{d}\t{e}");
}

#[allow(dead_code)]
mod math_const {
    pub(crate) const PI: f64 = 3.141592653589793;
    pub(crate) const PI_CUBE: f64 = PI * PI * PI;
}

fn _let_2() {
    println!("PI = {}", math_const::PI);
    println!("PI^3 = {}", math_const::PI_CUBE);
}

fn _let_3(exc: bool) -> std::io::Result<()> {
    use std::io::Write;

    pub trait Writer {
        fn write(&self) -> std::io::Result<()>;
        fn flush(&self) -> std::io::Result<()>;
    }

    let let_3_var: f64 = 64_f64;
    let str: String = format!("is format macros {}", let_3_var);
    let mut output = std::io::stdout();
    output.write_all(str.as_bytes()).unwrap();
    println!("8 : {}", let_3_var.sqrt());
    println!("0 : {}", (0_f64).sqrt());
    println!("NAN : {}", (-let_3_var).sqrt());

    if exc {
        // eprint equivalent to the print, but output goes to io::stderr
        eprintln!("Error: Could not complete");
        let error = std::io::Error::new(std::io::ErrorKind::Other, "MyError");
        return Err(error);
    } else {
        return Ok(());
    }
}

fn _let_4() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", x);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one: u8 = x.2;
    println!("{:?}", x);
    println!("1: {}, 2: {}, 3: {}", five_hundred, six_point_four, one);
}

fn _let_5() -> i32 {
    fn let_5_fn(num_a: i32, num_b: i32) -> i32 {
        let c = num_a * num_b;
        return c;
    }

    let x = {
        let y = 4;
        y * 2
    };
    let a: i32 = {
        let b = 8;
        let res = b * 12;
        res
    };
    return let_5_fn(a, x);
}

fn _let_6(num: i32) -> bool {
    return num > (i8::MAX / 2).into();
}

fn _let_7(num: i32) {
    let average = i8::MAX / 2;

    if (num > average.into()) {
        println!("Input: {}, average: {}", num, average);
    } else if num < average.into() {
        println!("Input number less that {}", average);
    } else {
        println!("Input number equal {}", average);
    }
}

fn _let_8(num: i32) {
    let _average: i32 = (i8::MAX / 2).into();
    match num {
        // 0..=average => println!("Input number less that {}", average),
        // 0..= => {println!("Input number more that {}", average);},
        average => println!("AVERAGE: Input number equal {}", average),
        _ => println!("We learn {}", "Rust"),
    }
}

fn _let_9() {
    let mut count: i32 = 0;
    'loop_name: loop {
        let mut remaining: i32 = 10;
        loop {
            if remaining == 9 {
                println!("Break child loop");
                break;
            }
            if count == 2 {
                println!("Break main loop");
                break 'loop_name;
            }
            remaining -= 1;
        }
        count += 1;
    }
    'name_for: for el in 1..5 {
        if el == 2 {
            break 'name_for;
        }
        println!("{el}");
    }
}

fn _let_10() {
    // _ <- write before var name for ignore compiler warning massage
    for _number in (1..10).rev() {
        println!("this step");
    }
}

fn _let_11() {
    let course = ("Rust", "beginner", "course");

    if let ("Rust", "beginner", cur) = course {
        println!("Match found, {cur}");
    } else {
        println!("Match not found");
    }
}

fn _let_12_types() {
    fn ex_tuple() {
        let boy_1: (String, u8, f32) = ("Mikhail".to_string(), 24, 36.6);
        let girl_1: (String, u8, f32, &str) = ("Vika".to_string(), 19, 37.1, "Sick");
        println!("{:?}", boy_1);
        println!("{:?}", girl_1);

        println!("{}", girl_1.3);
        let (_name, _age, _temperature) = boy_1;
    }
    ex_tuple();

    fn ex_array() {
        let shop_1: [&str; 5] = ["milk", "bread", "cake", "grain", "cheese"];
        let shop_2: [&str; 2] = ["int"; 2];

        println!("{:?}", shop_1);
        println!("{:?}", shop_2);

        // exception
        // let var_size_1: usize = 2;
        // let shop_3: [&str; 2] = ["int"; var_size];

        const VAR_SIZE: usize = 4;
        let shop_3: [&str; VAR_SIZE] = ["int"; VAR_SIZE];

        println!("{}", shop_3[0]);

        for el_index in 0..shop_3.len() {
            print!("{} ", shop_3[el_index]);
        }

        for el in shop_3 {
            print!("{} ", el);
        }

        println!();

        for (element_index, element) in shop_3.iter().enumerate() {
            println!("#{}: {}", element_index, element);
        }

        let slice_shop_1: &[&str] = &shop_1[0..3];
        println!("Shop slice: {:?}", slice_shop_1);
    }
    ex_array();

    fn ex_vector() {
        let _blank_vector: Vec<i32> = Vec::new();
        let shopping_1: Vec<&str> = vec!["bread", "rice", "milk", "cheese", "pineapple", "noodle"];
        println!("{:?}", shopping_1);
        println!("{}", shopping_1[1]);

        {
            // if you do not use .iter (), then there will be an exception below
            for el in shopping_1.iter() {
                print!("{} ", el);
            }

            println!("Vector length: {}", shopping_1.len());
            // see more about  ownership in rust
        }

        let mut shopping_2: Vec<&str> = vec!["ice", "water", "juice", "beer", "milk"];
        shopping_2.push("butter");
        // Oh no, there should not be "butter" and "ice"
        let _butter_str = shopping_2.pop().unwrap();

        if shopping_2.contains(&"ice") {
            let ice_index = shopping_2
                .iter()
                .position(|&el| el == "ice".to_string())
                .unwrap();
            shopping_2.remove(ice_index);
        }

        println!("{:?}", shopping_2);
    }
    ex_vector();

    fn ex_string() {
        let _blank_shop_1: String = String::new();
        let mut shop_1: String = String::from("OBravo");
        let mut shop_2: String = "7Days".to_string();

        shop_1.push(' ');
        shop_2.push_str(" ");

        let _shop_1 = shop_1.trim();
        let _shop_2 = shop_2.trim();
    }
    ex_string();

    fn ex_type_cast() {
        let a: i32 = 15;
        let b: f64 = (a as f64).into();
        println!("{b}");

        let cast_string_1: String = String::from("New string");
        let cast_string_2: &str = cast_string_1.as_str();
        println!("{cast_string_2}");

        let mut num: u8 = 32;
        for _ in 33..97 {
            num += 1;
            let character: char = num as char;
            println!("Number {} in char = {}", num, character);
        }
    }
    ex_type_cast()
}

fn _let_13_file_system() {
    fn open() -> std::io::Result<()> {
        use std::fs::File;

        File::open("foo.txt")?;
        return Ok(());
    }

    if let Err(e) = open() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn _let_14() {
    const SPACE: u8 = b' ';
    let example_string: String = String::from("hello world, TakiMoysha");

    {
        fn slice(s: &String) -> usize {
            let bytes = s.as_bytes(); // string to bytes array

            for (index, &item) in bytes.iter().enumerate() {
                if item == SPACE {
                    return index;
                }
            }

            return s.len();
        }
        let inx: usize = slice(&example_string);
        if inx == 11 || inx == 5 {
            println!("Space in {}", inx);
        }
    }

    {
        fn string_slice(s: &String) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == SPACE {
                    return &s[0..i];
                }
            }
            return &s[..];
        }
        let slice = string_slice(&example_string);
        println!("StringSlice: {}", slice);
    }
}

fn _let_15_struct() {
    {
        #[derive(Debug)]
        struct Point {
            x: f64,
            y: f64,
            z: f64,
            active: bool,
        }

        const DEFAULT_ACTIVE: bool = false;

        fn new_point(x: f64, y: f64, z: f64, active: bool) -> Point {
            return Point { x, y, z, active };
        }
        fn new_default_point(x: f64, y: f64, z: f64) -> Point {
            return Point {
                x,
                y,
                z,
                active: DEFAULT_ACTIVE,
            };
        }

        let p1 = new_default_point(1.0, 2.0, 11.0);
        let p2 = Point { x: 2.0, ..p1 };

        println!("p1: {:#?}", p1);
        println!("p2: {:#?}", p2);
    }

    {
        struct User {
            id: String,
            username: String, // String т.к. это владеющий тип, каждый экземпляр должен владеть всеми своими данными
            email: String,
            active: bool,
        }

        struct UserRef<'id, 'username> {
            ref_id: &'id str, // &str - хранит ссылку на данные, принадлежащие кому-то другому, смотри Rust Live time
            username: &'username str,
        }
    }

    {
        struct AlwaysEqual;

        let subject = AlwaysEqual; // instance of AlwaysEqual
    }
}

fn _let_16_rectangle() {
    fn log(area: u32) {
        println!("The area of the rectangle is {} square pixels.", area);
    }

    {
        fn area(width: u32, height: u32) -> u32 {
            return width * height;
        }
        let width1 = 30;
        let height1 = 50;
        log(area(width1, height1));
    }

    {
        fn area(dimensions: (u32, u32)) -> u32 {
            return dimensions.0 * dimensions.1;
        }
        let rect_1 = (30, 50);
        log(area(rect_1));
    }

    {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        // не нужно передавать сам объект, т.к. тогда переменная перестанет им обладать
        fn area(rectangle: &Rectangle) -> u32 {
            return rectangle.height * rectangle.width;
        }
        let rect_2 = Rectangle {
            width: 10,
            height: 20,
        };
        log(area(&rect_2));
    }

    {
        #[derive(Debug)]
        struct Example {
            x: i32,
            y: i32,
        }
        let ex = Example { x: 10, y: 15 };
        println!("{:?}", ex);
        println!("{:#?}", ex);
        dbg!(ex);
    }
}

fn _let_17_methods() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        // ------------ associated function
        // ============ methods
        fn area(self: &Self) -> u32 {
            return self.width * self.height;
        }
        fn expand(&mut self, size: u32) {
            self.width = self.width + size;
            self.height = self.height + size;
        }

        fn can_hold(&self, rectangle: &Self) -> bool {
            return self.width > rectangle.width && self.height > rectangle.height;
        }
        // ============

        fn new(x: u32, y: u32) -> Self {
            return Self {
                width: x,
                height: y,
            };
        }
        fn square(size: u32) -> Self {
            return Self {
                width: size,
                height: size,
            };
        }
        // ------------
    }

    impl Rectangle {
        fn width(&self) -> bool {
            return self.width > 0;
        }
        fn height(&self) -> bool {
            return self.height > 0;
        }
    }

    let mut rect_1 = Rectangle {
        width: 30,
        height: 20,
    };
    println!("The Area is {}", rect_1.area());
    rect_1.expand(10);
    println!("The Area is {}", rect_1.area());

    let rect_2 = Rectangle::new(10, 20);

    println!("rect_1 can hold rect_2 {}", rect_1.can_hold(&rect_2));
}

fn _let_18_enums() {
    enum IpAddrKind {
        V4,
        V6,
    }
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    fn route(ip_kind: IpAddrKind) {}
    route(four);
    route(six);

    enum IpAddr1 {
        V4(String),
        V6(String),
    }
    let _home = IpAddr1::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr1::V6(String::from("::1"));

    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let _home = IpAddr2::V4(127, 0, 0, 1);
    let _loopback = IpAddr2::V6(String::from("::1"));

    enum MyOption<T> {
        None,
        Some(T),
    }
    let some_number = Some(5);
    assert_eq!(some_number.is_some(), true);
    let some_char = Some('e');
    assert_eq!(some_char.is_some(), true);
    // let absent_number: MyOption<u32> = MyOption::from(None);
    // assert_eq!(absent_number.is_some(), false);
}

fn _let_18_1_option() {
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);

    let x: Option<u32> = None;
    assert_eq!(x.is_none(), true);
}

fn _let_19_match() {
    struct Cent(u8);
    #[derive(Debug)]
    enum UsState {
        Alaska,
        Alabama,
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> Cent {
        return match coin {
            Coin::Penny => Cent(1),
            Coin::Nickel => Cent(5),
            Coin::Dime => Cent(10),
            Coin::Quarter(state) => {
                println!("Quarter state is {:?}", state);
                Cent(25)
            }
        };
    }
    assert_eq!(value_in_cents(Coin::Penny).0, Cent(1).0);
    assert_eq!(value_in_cents(Coin::Nickel).0, Cent(5).0);
    assert_eq!(value_in_cents(Coin::Dime).0, Cent(10).0);
    assert_eq!(
        value_in_cents(Coin::Quarter(UsState::Alabama)).0,
        Cent(25).0
    );

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => (),
        7 => {}
        value => println!("{}", value),
    }
}

fn _let_19_if_let() {
    let config_max = Some(3u8);
    let msg = "This maximum is configured to be";
    match config_max {
        Some(max) => println!("{} {}", msg, max),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("{} {}", msg, max);
    } else {
        println!("It will not be written");
    }
}

fn _let_20_same_name() {
    use std::fmt;
    use std::io;

    // fn fun1() -> fmt::Result { ... }

    // fn fun2() -> io::Result<()> { ... }

    use std::io as IoResult;

    // fn fun3() -> IoResult<()> { ... }
}

fn _let_20_1_re_export() {
    use crate::garden::vegetables::Barn;

    pub fn eat_at_restaurant() {
        Barn::add_vegetables();
    }
}

fn _let_20_2_external_packets() {
    use rand::Rng;

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{:?}", secret_number);
}

fn _let_21_vector() {
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(5);

    let mut v2 = vec![1, 2, 3];
    v2.push(99);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => {
            println!("There is no third element");
            v.push(102);
        }
    }

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    print!("[");
    for i in &v2 {
        print!("{i}, ");
    }
    println!("]");

    for i in &mut v2 {
        *i += 50;
    }
    println!("{v2:?}");

    // Vec cat store the values of the same type
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn _let_22_string() {
    let data = "Init content";
    let string_data = String::from("Init content");
    let s = data.to_string();
    let s = "Init content".to_string();

    let s = "Hello".chars();
    for char in s {
        print!("{}", char);
    }
    println!();

    println!("{}", &data[0..8]);
}

fn _let_23_hashmap() {
    use std::collections::HashMap;

    struct CommandName(String);
    struct CommandScore(i32);

    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 10);

    let team_name = String::from("Blue");
    let team_score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{} - {}", team_name, team_score);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);
}

fn _let_23_hashmap_exercise() {
    let list = [51, 345, 83, 1, 4, 589, 32, 51];
    // todo: search: average, median (after sort), mode of list

    let my_string = String::from("This example string needs to be translated into pig latin.");
    // todo: Первая согласная каждого слова перемещается в конец и к ней добавляется окончание "ay", так "first" станет "irst-fay". Слову, начинающемуся на гласную, в конец добавляется "hay" ("apple" становится "apple-hay").

    // todo: create text-interface that allows the user to add employee names to company department name. "Add Sally to Engineering" or "Add Amir to Sales". Then let the user get a list of all people in a department, or all people in a company, sorted alphabetically by department.
}

fn _let_24_exceptions() {}

fn _let_25_generics() {
    let number_list = vec![34, 61, 13, 671, 23];
    let char_list = vec!['a', 'w', 'b', 'g', '1', 'k'];

    fn largest_vec(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    let res_largest_vec = largest_vec(&number_list);

    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    let res_largest_char = largest_char(&char_list);

    fn largest_generic<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    // assert_eq!(res_largest_vec, largest_generic(number_list));
    // assert_eq!(res_largest_char, largest_generic(char_list));

    struct Point<T> {
        x: T,
        y: T,
    }
    fn test_point() {
        let p1 = Point { x: 5, y: 2 };
        let p2 = Point { x: 5.1, y: 10.5 };
        let p3 = Point { x: 'a', y: 'b' };
    }
}

fn _let_26_traits() {
    pub trait Summary {
        fn summarize(&self) -> String;
        fn default_summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("visor"),
        content: String::from("Hello Dj"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: `{}`", tweet.summarize());
    println!("1 new tweet: `{}`", tweet.default_summarize());

    use std::fmt::{Debug, Display};
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        32
    }

    fn returns_summarize() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("hello world"),
            reply: false,
            retweet: false,
        }
    }
}

fn _let_27_tests() {}
// tests
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn _let_28_functionality() {
    mod closures {
        #[derive(Debug, PartialEq, Copy, Clone)]
        pub enum ShirtColor {
            Red,
            Blue,
        }

        pub struct Inventory {
            pub shirts: Vec<ShirtColor>,
        }

        impl Inventory {
            pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
                user_preference.unwrap_or_else(|| self.most_stocked())
            }

            fn most_stocked(&self) -> ShirtColor {
                let mut num_red = 0;
                let mut num_blue = 0;

                for color in &self.shirts {
                    match color {
                        ShirtColor::Red => {
                            num_red += 1;
                        }
                        ShirtColor::Blue => {
                            num_blue += 1;
                        }
                    }
                }
                return if num_red > num_blue {
                    ShirtColor::Red
                } else {
                    ShirtColor::Blue
                };
            }
        }
    }

    let store = closures::Inventory {
        shirts: vec![
            closures::ShirtColor::Red,
            closures::ShirtColor::Blue,
            closures::ShirtColor::Blue,
        ],
    };
    let user_pref_1 = Some(closures::ShirtColor::Red);
    let giveaway_1 = store.giveaway(user_pref_1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref_1, giveaway_1
    );

    let user_pref_2 = None;
    let giveaway_2 = store.giveaway(user_pref_2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref_2, giveaway_2
    );

    pub mod iterators {
        #[derive(PartialEq, Debug)]
        pub struct Shoe {
            pub size: u32,
            pub style: String,
        }

        pub fn shoes_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
            shoes.into_iter().filter(|s| s.size == size).collect()
        }
    }

    let shoes = vec![
        iterators::Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        iterators::Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        iterators::Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = iterators::shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            iterators::Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            iterators::Shoe {
                size: 10,
                style: String::from("boot")
            }
        ]
    );
}

fn _let_29_box_ref_links() {
    #[derive(Debug)]
    enum MsgType {
        msg,
        service,
    }

    #[derive(Debug)]
    struct Msg {
        id: i32,
        t: MsgType,
        message: String,
    }

    let m = Msg {
        id: 1,
        t: MsgType::service,
        message: "echo".to_string(),
    };

    println!("{:?}", m);
}

fn _let_29_1_recursive() {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
}

fn _let_29_2_deref() {
    let x = 5;
    let y = &x;
    let y_b = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *y_b);

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    use std::ops::Deref;

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let y_1 = MyBox::new(x);
    assert_eq!(5, *y_1);
}

use std::{mem::drop, rc::Rc};
fn _let_29_3_drop() {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: String::from("c stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("d stuff"),
    };

    println!("CustomSmartPointer created.");

    drop(c);
    println!("CustomSmartPointer 'c' destroy.");
}

fn _let_29_4_rc() {
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use std::rc::Rc;

    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    let b = List::Cons(3, Rc::clone(&a));
    let c = List::Cons(4, Rc::clone(&a));

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    println!("count: {}", Rc::strong_count(&a));
    {
        let d = Rc::clone(&a);
        println!("count: {}", Rc::strong_count(&a));
    }
    println!("count: {}", Rc::strong_count(&a));
}

fn _let_30_refcell() {
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }
    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;
            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quouta!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of you quota.");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75 of your quota!");
            }
        }
    }

    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        println!("messeges {:?}", mock_messenger.sent_messages.borrow());
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    it_sends_an_over_75_percent_warning_message();

    use List::{Cons, Nil};
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    fn demonstrate_ref_cell_1() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));
        // println!("a next item = {:?}", a.tail()); // stack overflow (cycle link)
    }

    println!("Demonstrate ref cell ----- ");
    demonstrate_ref_cell_1();
    println!("-------------------------- ");

    fn demonstrate_rc_weak_1() {
        use std::cell::RefCell;
        use std::rc::{Rc, Weak};

        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }

        fn main() {
            let leaf = Rc::new(Node {
                value: 3,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
            });

            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None

            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        }
    }

    println!("Demonstrate weak --------- ");
    demonstrate_rc_weak_1();
    println!("-------------------------- ");
}

use std::time::Duration;
use std::{string, thread};
fn _let_31_multithreading() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi thread {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(2));
    }
    handle.join().unwrap();

    let v = vec![1, 2, 3];
    // move take v to ownership
    let handle_with_move = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle_with_move.join().unwrap();

    // ThreadPool
    pub struct ThreadPool {
        workers: Vec<Worker>,
    }
    pub struct Worker {
        id: usize,
        thread: thread::JoinHandle<()>,
    }
    impl Worker {
        fn new(id: usize) -> Worker {
            let thread = thread::spawn(|| {
                thread::sleep(Duration::from_secs(60));
            });

            Worker { id, thread }
        }
    }

    impl ThreadPool {
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0);
            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(Worker::new(id));
            }

            ThreadPool { workers }
        }
    }

    let tp = ThreadPool::new(8);
    thread::sleep(Duration::from_secs(60));
}

use std::sync::mpsc;
fn _let_31_1_channels() {
    let (tx, rx) = mpsc::channel(); // multiple producer, single consumer

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    println!("Whait value...");
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn _let_31_2_safe_channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // can't print, val was send
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn _let_31_3_multi_send() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread!"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn _let_31_4_clone_producer() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            String::from("."),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("msg-s"),
            String::from("for"),
            String::from("you"),
            String::from("."),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

use std::sync::{Arc, Mutex};
fn _let_32_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn _let_32_1_mutex_in_threads() {
    // let count = Mutex::new(0); // nope
    // let count = Rc::new(Mutex::new(0)); // also no
    let count = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let count = Arc::clone(&count);
        let handle = thread::spawn(move || {
            let mut num = count.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *count.lock().unwrap());
}

fn _let_33_1_oop() {
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }
}

fn _let_33_2_trait_instead_inheritance() {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {}
    }

    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {}
    }

    let screen = Screen {
        components: vec![Box::new(Button {
            width: 5,
            height: 10,
            label: String::from("Hello"),
        })],
    };

    fn start_my_gui() {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("yes"),
                        String::from("maybe"),
                        String::from("no"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("Ok"),
                }),
            ],
        };
    }

    start_my_gui();
}

fn _let_33_3_state_pattern() {
    trait PostState {
        fn request_review(self: Box<Self>) -> Box<dyn PostState>;
        fn approve(self: Box<Self>) -> Box<dyn PostState>;
        fn reject(self: Box<Self>) -> Box<dyn PostState> {
            Box::new(Draft {})
        }
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }

    struct Draft {}
    struct Published {}
    struct PendingReview {
        content: String,
    }

    struct Post {
        state: Option<Box<dyn PostState>>,
        content: String,
    }

    impl PostState for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn PostState> {
            Box::new(PendingReview {
                content: String::from(""),
            })
        }
        fn approve(self: Box<Self>) -> Box<dyn PostState> {
            self
        }
    }

    impl PostState for Published {
        fn request_review(self: Box<Self>) -> Box<dyn PostState> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn PostState> {
            self
        }
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }

    impl PostState for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn PostState> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn PostState> {
            self
        }
        fn reject(self: Box<Self>) -> Box<dyn PostState> {
            Box::new(Draft {})
        }
    }

    impl Post {
        pub fn new(&self) -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }
        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }
        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }
    }
}

fn _let_34_1_patterns_and_matching() {
    // literals, destructured [arrays, enums, structs, tuples], variables, wildcard, placeholders

    const VALUE: u32 = 0;
    const PATTERN: u32 = 0;
    const EXPRESSION: u32 = 0;

    // match sould be e..austive (that all possibilities for the VALUE)
    // think of `match` as an assignable value. `match` follows the same rules
    let x = match VALUE {
        // PATTERN => EXPRESSION,
        0 => Some(0 as i32),
        101..=199 => Some((VALUE * 2) as i32),
        1..=100 | 200..=1000 => Some(VALUE as i32),
        _ => Some(-1), // any pattern
    };

    // not check exhaustive (as match)
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Usin purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

pub fn run() {
    println!("Rust book - start exercises.");
    _let_34_1_patterns_and_matching();
    println!("Rust book - end exercises.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _let_27_it_can_hold() {
        let r_1 = Rectangle {
            width: 10,
            height: 20,
        };
        let r_2 = Rectangle {
            width: 5,
            height: 15,
        };
        assert!(r_1.can_hold(&r_2));
        assert_eq!(r_1.can_hold(&r_2), true);
    }

    #[test]
    fn _let_27_it_not_can_hold() {
        let r_1 = Rectangle {
            width: 5,
            height: 1,
        };
        let r_2 = Rectangle {
            width: 15,
            height: 20,
        };
        assert!(!r_1.can_hold(&r_2));
        assert_ne!(r_1.can_hold(&r_2), true);
    }

    #[test]
    #[ignore = "always failed"]
    fn _let_27_failed_test() {
        let result = String::from("Hello world!");
        assert!(false, "Example custom exception msg: <{}>", result);
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn _let_27_planned_panic() {
        fn is_right(value: i32) -> bool {
            if value < 1 {
                panic!("Value must be greater than or equal to 1, got {}", value);
            } else if value > 100 {
                panic!("Value must be less than or equal to 100, got {}.", value);
            }
            return true;
        }

        is_right(120);
        // is_right(-1); bad panic
    }

    #[test]
    fn _let_27_result_in_test() -> Result<(), String> {
        // в таких тестах можно использовать оператор ? в тестах, если какая-то ф-ция внутри завершилась паникой
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }

    #[test]
    fn _let_28_iterator_sum() {
        let v1 = vec![1, 2, 3, 4];

        let total: i32 = v1.iter().sum();
        assert_eq!(total, 10);

        let mut sum = 0;
        v1.iter().for_each(|el| {
            sum = sum + el;
        });
        assert_eq!(sum, 10);

        let mut sum = 0;
        for el in v1.iter() {
            sum = sum + el;
        }
        assert_eq!(sum, 10);
    }
}
