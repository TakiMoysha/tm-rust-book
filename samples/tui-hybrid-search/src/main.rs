use thiserror::Error;

pub mod infra;
pub mod tui;

use ratatui_themes::ThemeName;

fn get_theme() -> ThemeName {
    // ThemeName::Cyberpunk
    ThemeName::MonokaiPro
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    infra::database::bootstrap().await?;

    let bad_value = infra::database::DB
        .query("CREATE document SET title = 'foo', extract = 'bar', level = 9")
        .await?
        .check();

    println!("bad_value: {bad_value:?}");
    Ok(())
}
