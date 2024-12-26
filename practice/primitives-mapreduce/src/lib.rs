use std::thread;

fn summing(c: &char) -> u32 {
    c.to_digit(10).expect("should be a digit")
}

pub fn parallel_sum(input: &'static str) -> u32 {
    let mut children = vec![];

    for (i, data_l) in input.lines().enumerate() {
        // println!("data segment {} is \"{}\"", i, data_l);

        children.push(thread::spawn(move || -> u32 {
            let result = data_l.chars().map(|c| summing(&c)).sum();
            // println!("processed segment {}, result={}", i, result);
            result
        }))
    }

    children.into_iter().map(|c| c.join().unwrap()).sum::<u32>()
}

pub fn single_thread_sum(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.chars().map(|c| summing(&c)).sum::<u32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &str = "86967897737416471853297327050364959\n11861322575564723963297542624962850\n70856234701860851907960690014725639\n38397966707106094172783238747669219\n52380795257888236525459303330302837\n58495327135744041048897885734297812\n69920216438980873548808413720956532\n16278424637452589860345374828574668";

    #[test]
    fn test_single_thread_sum() {
        single_thread_sum(DATA);
        assert_eq!(single_thread_sum(DATA), 1342);
    }
}
