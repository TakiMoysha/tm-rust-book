use playground_se::helpers::{env_processing, glob_sbc, parse_sbc_definitions};

fn main() -> anyhow::Result<()> {
    let env_vars = env_processing()?;
    let sbc_files = glob_sbc(&env_vars.game_cubes_dir())?;

    sbc_files.iter().for_each(|sbc_file| {
        if let Ok(sbc_definitions) = parse_sbc_definitions(sbc_file) {
            let filtered = sbc_definitions
                .iter()
                .filter(|definition| definition.display_name.contains("Battery"));

            for def in filtered {
                println!(
                    "\n--- DEF {:?} [{}/{}] ---\n\t{}",
                    def.id,
                    def.display_name,
                    def.cube_size,
                    sbc_file.display()
                );
            }
        }
    });

    Ok(())
}
