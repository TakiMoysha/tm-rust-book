use serde::{Deserialize, Serialize};
use std::{env, sync::LazyLock};

use surrealdb::types::SurrealValue;
use surrealdb::{Surreal, engine::any::Any};

pub static DB: LazyLock<Surreal<Any>> = LazyLock::new(Surreal::init);

#[derive(Debug)]
pub struct DbGuard;

// impl Drop for DbGuard {
//     fn drop(&self) {}
// }

#[derive(Debug, Serialize, Deserialize, SurrealValue)]
pub struct Person {
    name: String,
    role: String,
}

async fn define_schema() -> anyhow::Result<()> {
    DB.query(sql_init()).await?.check()?;

    Ok(())
}

pub async fn bootstrap() -> anyhow::Result<DbGuard> {
    let endpoint = env::var("DB_ENDPOINT").unwrap_or("mem://".into());

    // URL scheme picks the engine: mem:// embeds, ws:// needs a server.
    // A turbofish like connect::<Ws>() would FORCE websocket transport.
    DB.connect(endpoint).await?;
    DB.use_ns("tui-explorer").use_db("dev").await?;

    define_schema().await?;

    Ok(DbGuard)
}

fn sql_init() -> &'static str {
    r#"
    DEFINE TABLE IF NOT EXISTS document SCHEMAFULL;
    DEFINE FIELD extract ON document TYPE string;
    DEFINE FIELD title ON document TYPE string;
    DEFINE FIELD mistral_embedding ON document TYPE option<array<float>> DEFAULT [];
    DEFINE FIELD openai_embedding ON document TYPE option<array<float>> DEFAULT [];
    DEFINE ANALYZER en_analyzer TOKENIZERS class FILTERS lowercase,edgengram(3,10);
    DEFINE INDEX en_extract ON document FIELDS extract FULLTEXT ANALYZER en_analyzer BM25 HIGHLIGHTS;
    DEFINE INDEX en_title ON document FIELDS title FULLTEXT ANALYZER en_analyzer BM25 HIGHLIGHTS;

    DEFINE TABLE link TYPE RELATION IN document OUT document ENFORCED;

    DEFINE INDEX only_one_link ON link FIELDS in,out UNIQUE;"#
}
