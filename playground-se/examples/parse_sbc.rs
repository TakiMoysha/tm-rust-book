use std::{
    env,
    fs::{self, read_to_string},
    path::PathBuf,
};

use anyhow::Context;

use playground_se::types;

fn main() -> anyhow::Result<()> {
    let gamedir = env::var("GAME_DIR")
        .expect("GAME_DIR must be set (SE game dir)")
        .parse::<PathBuf>()
        .context("failed to parse GAME_DIR")?;
    let sdkdir = env::var("SDK_DIR")
        .expect("SDK_DIR must be set (SE sdk dir)")
        .parse::<PathBuf>()
        .context("failed to parse SDK_DIR")?;

    let data_content_dir = gamedir.join("Content/Data/CubeBlocks");
    let origin_content_dir = sdkdir.join("OriginalContent/Models");

    println!(
        "GAME_DIR \t{}: \t{}",
        data_content_dir.exists(),
        data_content_dir.display()
    );
    println!(
        "SDK_DIR \t{}: \t{}",
        origin_content_dir.exists(),
        origin_content_dir.display()
    );

    let sbc_files: Vec<_> = fs::read_dir(data_content_dir.clone())
        .expect("failed to read data dir")
        .filter_map(|entry| {
            if let Ok(e) = entry {
                if e.path().is_file() && e.path().extension().is_some_and(|ext| ext == "sbc") {
                    Some(e.path())
                } else {
                    None
                }
            } else {
                eprintln!("failed to read entry: {}", entry.unwrap_err());
                None
            }
        })
        .collect();

    if sbc_files.is_empty() {
        println!("No SBC files found in {:?}", data_content_dir);
        return Ok(());
    }

    let content = read_to_string(&sbc_files[0]).context("failed to read file")?;

    let definitions = types::parse_sbc(&content)?.cube_blocks.definitions;
    println!(
        "\nParsed {} definitions from {}",
        definitions.len(),
        sbc_files[0].display()
    );
    println!("---\n");
    for (i, def) in definitions.iter().enumerate() {
        println!("\n--- Definition {} ---", i + 1);
        println!("ID: {:?}", def.id);
        println!("DisplayName: {}", def.display_name);
        println!("CubeSize: {}", def.cube_size);
        println!("Size: {:?}", def.size);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_test_sbc_file() {
        let example_file = Path::new("tmp/example.sbc");

        if let Ok(content) = read_to_string(example_file) {
            let definitions = types::parse_sbc(&content);
            assert!(definitions.is_ok(), "Failed to parse example.sbc");

            println!("Successfully parsed {:?} definitions", definitions.as_ref());
            for (i, def) in definitions
                .unwrap()
                .cube_blocks
                .definitions
                .iter()
                .enumerate()
            {
                println!("\n--- Definition {} ---", i + 1);
                println!("ID: {:?}", def.id);
                println!("DisplayName: {}", def.display_name);
                println!("CubeSize: {}", def.cube_size);
                println!("Size: {:?}", def.size);
            }
        } else {
            panic!("Failed to read example.sbc");
        }
    }
}
