use playground_se::{
    helpers::{env_processing, glob_sbc, parse_sbc_file},
    types::Definition,
};

fn main() -> anyhow::Result<()> {
    let env_vars = env_processing()?;
    let sbc_files = glob_sbc(&env_vars.game_cubes_dir())?;

    sbc_files.iter().for_each(|sbc_file| {
        if let Ok(sbc_definitions) = parse_sbc_file(sbc_file) {
            let filtered = sbc_definitions
                .iter()
                .filter(|definition| definition.display_name.contains("Battery"));

            for def in filtered {
                println!(
                    "\n--- DEF {:?} [{}/{}] ---",
                    def.id, def.display_name, def.cube_size
                );
            }
        }
    });

    Ok(())
}
