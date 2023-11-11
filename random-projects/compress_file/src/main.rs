use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::process::exit;

struct CompressOpts {
    input_file: String,
    output_file: String,
}

fn parse_args() -> Result<CompressOpts, String> {
    if args().len() < 3 {
        println!("Usage: compress_file <input_file> <output_file>");
        return Result::Err("Invalid arguments".to_string());
    }

    let input_file = args().nth(1).unwrap();
    let output_file = args().nth(2).unwrap();
    Ok(CompressOpts {
        input_file,
        output_file,
    })
}

fn main() {
    let opts = parse_args().unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1)
    });

    let input_file = Box::new(File::open(opts.input_file).unwrap_or_else(|_| {
        eprintln!("Failed to open {}", args().nth(1).unwrap());
        exit(1)
    }));
    let output_file = Box::new(File::create(opts.output_file).unwrap_or_else(|_| {
        eprintln!("Failed to create {}", args().nth(2).unwrap());
        exit(1)
    }));

    let mut input_buffer = BufReader::new(input_file.as_ref());
    let mut encoder = flate2::write::GzEncoder::new(output_file.as_ref(), flate2::Compression::default());
    let start_time = std::time::Instant::now();

    std::io::copy(&mut input_buffer, &mut encoder).unwrap();

    println!(
        "Compressed {} to {}",
        input_file.metadata().unwrap().len(),
        output_file.metadata().unwrap().len()
    );
    println!("Elapsed time: {:.2?}", start_time.elapsed());

    exit(0);
}
