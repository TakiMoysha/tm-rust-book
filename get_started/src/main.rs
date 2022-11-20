#[warn(dead_code)]
fn _let_1() {
    let (a, b, c, d, e): (i8, i16, i32, i64, i128) = (
        i8::MAX,
        i16::MAX,
        i32::MAX,
        i64::MAX,
        i128::MAX,
    );
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
    if num > average.into() {
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
            return Point { x, y, z, active: DEFAULT_ACTIVE };
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
    fn _log(area: u32) {
        println!("The area of the rectangle is {} square pixels.", area);
    }

    {
        fn area(width: u32, height: u32) -> u32 {
            return width * height;
        }
        let width1 = 30;
        let height1 = 50;
        _log(area(width1, height1));
    }

    {
        fn area(dimensions: (u32, u32)) -> u32 {
            return dimensions.0 * dimensions.1;
        }
        let rect_1 = (30, 50);
        _log(area(rect_1));
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
        let rect_2 = Rectangle { width: 10, height: 20 };
        _log(area(&rect_2));
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
            return Self { width: x, height: y };
        }
        fn square(size: u32) -> Self {
            return Self { width: size, height: size };
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

    let mut rect_1 = Rectangle { width: 30, height: 20 };
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
    assert_eq!(value_in_cents(Coin::Quarter(UsState::Alabama)).0, Cent(25).0);

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

mod garden;
fn _let_20_1_re_export() {
    use garden::vegetables::Barn;

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
        Text(String)
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12)
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

    let mut scores:HashMap<String, u32> = HashMap::new();
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

fn main() {
    _let_23_hashmap();
}