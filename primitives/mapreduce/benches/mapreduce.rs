use mapreduce::{parallel_sum, single_thread_sum};

fn main() {
    divan::main();
}

static DATA: &str = "86967897737416471853297327050364959\n11861322575564723963297542624962850\n70856234701860851907960690014725639\n38397966707106094172783238747669219\n52380795257888236525459303330302837\n58495327135744041048897885734297812\n69920216438980873548808413720956532\n16278424637452589860345374828574668";

#[divan::bench]
fn parallel() -> u32 {
    let res = divan::black_box(parallel_sum(DATA));
    // println!("parallel: {}", res);
    res
}

#[divan::bench]
fn single() -> u32 {
    let res = divan::black_box(single_thread_sum(DATA));
    // println!("single: {}", res);
    res
}

