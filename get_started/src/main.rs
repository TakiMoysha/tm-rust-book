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

fn _let_14()  {
    const SPACE: u8 = b' ';
    let example_string: String = String::from("hello world, mishko");

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


fn main() {
    _let_14();
}