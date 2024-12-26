use std::thread;

fn summing_line(line: &str) -> u32 {
    line.chars()
        .map(|c| c.to_digit(10).expect("should be a digit"))
        .sum()
}

// spawn a thread for each line
pub fn parallel_sum(input: &'static str) -> u32 {
    let mut children = vec![];

    input
        .lines()
        .for_each(|data_l| children.push(thread::spawn(move || -> u32 { summing_line(data_l) })));

    children.into_iter().map(|c| c.join().unwrap()).sum::<u32>()
}

pub fn single_thread_sum(input: &str) -> u32 {
    input.lines().map(summing_line).sum()
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
