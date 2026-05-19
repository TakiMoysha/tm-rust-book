use std::{
    env,
    fs::{self, read_to_string},
    path::PathBuf,
};

use anyhow::Context;

use crate::types;

pub struct EnvVariables {
    gamedir: PathBuf,
    sdkdir: PathBuf,
}

impl EnvVariables {
    /// Stored files with specification for game models (cubes)
    pub fn game_cubes_dir(&self) -> PathBuf {
        self.gamedir.join("Content/Data/CubeBlocks")
    }

    /// Stored Models and xml files for development
    pub fn sdk_models_dir(&self) -> PathBuf {
        self.sdkdir.join("OriginalContent/Models")
    }
}

pub fn env_processing() -> anyhow::Result<EnvVariables> {
    let gamedir = env::var("GAME_DIR")
        .expect("GAME_DIR must be set (SE game dir)")
        .parse::<PathBuf>()
        .context("failed to parse GAME_DIR")?;
    let sdkdir = env::var("SDK_DIR")
        .expect("SDK_DIR must be set (SE sdk dir)")
        .parse::<PathBuf>()
        .context("failed to parse SDK_DIR")?;

    Ok(EnvVariables { gamedir, sdkdir })
}

pub fn glob_sbc(dir: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    let sbc_files = fs::read_dir(dir)
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

    Ok(sbc_files)
}

pub fn parse_sbc_file(path: &PathBuf) -> anyhow::Result<Vec<types::Definition>> {
    let content = read_to_string(path).context("failed to read file")?;
    let sbc_definitions = types::parse_sbc(&content)?;
    Ok(sbc_definitions.cube_blocks.definitions)
}
