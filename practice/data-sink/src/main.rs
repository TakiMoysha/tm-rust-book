use sea_orm::entity::prelude::*;
use serde::Deserialize;

use anyhow::Result;
use clap::Parser;
use sea_orm::{
    ActiveModelTrait, ConnectOptions, ConnectionTrait, Database, DbConn, DbErr, IntoActiveModel,
    NotSet, Schema,
};
use sea_streamer::{Buffer, Consumer, Message, SeaStreamer, StreamKey, Streamer, StreamerUri};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Deserialize)]
#[sea_orm(table_name = "event")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(default)]
    pub id: i32,
    pub timestamp: String,
    pub bid: String,
    pub ask: String,
    pub bid_vol: String,
    pub ask_vol: String,
}

#[derive(Debug, Copy, Clone, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
#[derive(Debug, Parser)]
struct Args {
    #[clap(long, help = "Streamer URI", default_value = "redis://localhost:6379")]
    streamer: StreamerUri,
    #[clap(long, help = "Stream Key", default_value = "BTC_USD")]
    stream_key: StreamKey,
}

#[derive(Deserialize)]
struct Item {
    spread: Model,
}

async fn create_tables(db: &DbConn) -> Result<(), DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let stmt = builder.build(schema.create_table_from_entity(Entity).if_not_exists());
    log::info!("{stmt}");
    db.execute(stmt).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let Args {
        streamer,
        stream_key,
    } = Args::parse();

    let mut opt = ConnectOptions::new(format!("sqlite://{}.sqlite?mode=rwc", stream_key));
    opt.max_connections(1).sqlx_logging(false);
    let db = Database::connect(opt).await?;
    create_tables(&db).await?;

    let streamer = SeaStreamer::connect(streamer, Default::default()).await?;
    let consumer = streamer
        .create_consumer(&[stream_key], Default::default())
        .await?;

    loop {
        let message = consumer.next().await?;
        let payload = message.message();
        let json = payload.as_str()?;
        log::info!("{json}");
        let item: Item = serde_json::from_str(json)?;
        let mut spread = item.spread.into_active_model();
        spread.id = NotSet;
        spread.save(&db).await?;
    }
}
