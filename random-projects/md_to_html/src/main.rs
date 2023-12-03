use std::path::Path;


pub struct RuntimeOpts<'a> {
    pub sources_path: &'a Path,
}

fn main() {
    let sources_path = Path::new("data");
    validators::validate_sources(&sources_path);
}

pub mod validators {
    use std::path::Path;

    pub fn validate_sources(path: &Path) -> bool { path.is_dir() }
}

