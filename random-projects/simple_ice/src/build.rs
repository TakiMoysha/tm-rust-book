use ice_rs::slice::parser;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=build.rs");
  let ice_files = vec![
      String::from("ice/hello.ice")
  ];
  let root_module = parser::parse_ice_files(&input, ".")?;
  root_module.generate(Path::new("./src/gen"))
}

