use playground_se::helpers::{env_processing, glob_sbc};

fn main() -> anyhow::Result<()> {
    let env_vars = env_processing()?;

    Ok(())
}
