use std::fs;
use zip::read::ZipArchive;

struct DecompressOpts {
    input_file: String,
    output_file: Option<String>,
}

fn parse_args() -> DecompressOpts {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Bad function signature.\nUsage: compress_file <input_file> <output_file>");
    }

    DecompressOpts {
        input_file: args.get(1).unwrap().into(),
        output_file: args.get(2).map(|x| x.into()),
    }
}

fn main() {
    let opts = parse_args();

    let input_f_path = std::path::Path::new(&*opts.input_file);
    let destination_path_ = opts
        .output_file
        .unwrap_or(input_f_path.parent().unwrap().to_string_lossy().to_string());
    let destination_path = std::path::Path::new(&*destination_path_);
    let input_file = fs::File::open(&input_f_path).unwrap();
    let start_time = std::time::Instant::now();

    let mut archive = ZipArchive::new(input_file).unwrap();

    for i in 0..archive.len() {
        let mut arch_file = archive.by_index(i).unwrap();
        let outpath = match arch_file.enclosed_name() {
            Some(p) => destination_path.join(p).clone(),
            None => continue,
        };

        let comment = arch_file.comment();
        if !comment.is_empty() {
            println!("File {} comment: {}", i, comment);
        }

        if (*arch_file.name()).ends_with("/") {
            println!("Folder {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to '{}' ({} bytes)",
                i,
                outpath.display(),
                arch_file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            std::io::copy(&mut arch_file, &mut outfile).unwrap();
        }
    }

    println!("Elapsed time: {:?}", start_time.elapsed());
}
