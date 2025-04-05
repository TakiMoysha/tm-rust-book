use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;

use ::serde::Deserialize;
use ::serde::Serialize;
use futures::{StreamExt, stream};
use postgres_jobs::PostgresJob;
use postgres_jobs::PostgresJobStatus;
use queue::Queue;
use sqlx::Database;
use sqlx::types::Json;
use ulid::Ulid;
use uuid::Uuid;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| Error::from("DATABASE_URL is not set"))
        .unwrap();
    let db = db::connect(&database_url)
        .await
        .map_err(|_| Error::from("Failed to connect"))
        .unwrap();
    db::migrate(&db)
        .await
        .map_err(|_| Error::from("Failed to migrate"))?;

    let queue = Arc::new(PostgresQueue::new(db.clone()));
    let job = Message::SendSignInEmail {
        email: "test@gmail.edu".to_string(),
        name: "John Doe".to_string(),
        code: "000-000".to_string(),
    };

    tokio::time::sleep(Duration::from_secs(2)).await;

    Ok(())
}

pub mod db {
    use std::time::Duration;

    use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

    pub type DB = Pool<Postgres>;

    pub async fn connect(database_url: &str) -> Result<DB, crate::Error> {
        PgPoolOptions::new()
            .max_connections(10)
            .max_lifetime(Duration::from_secs(30 * 60))
            .connect(database_url)
            .await
            .map_err(|_| crate::Error::from("Failed to connect"))
    }

    pub async fn migrate(db: &DB) -> Result<(), crate::Error> {
        match sqlx::migrate!("./migrations").run(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
        .map_err(|_: sqlx::Error| crate::Error::from("Failed to migrate"))
        .unwrap();

        Ok(())
    }
}
const CONCURRENCY: usize = 10;

async fn run_worker(queue: Arc<dyn Queue>) {
    loop {
        let jobs = match queue.pull(CONCURRENCY as u32).await {
            Ok(jobs) => jobs,
            Err(err) => {
                println!("Failed to pull jobs: {}", err);
                tokio::time::sleep(Duration::from_secs(1)).await;
                Vec::new()
            }
        };

        let number_of_jobs = jobs.len();
        if number_of_jobs > 0 {
            println!("Pulled {} jobs", number_of_jobs);
        }

        stream::iter(jobs)
            .for_each_concurrent(CONCURRENCY, |job| async {
                let job_id = job.id;

                let res = match queue.delete_job(job_id).await {
                    Ok(()) => queue.delete_job(job_id).await,
                    Err(err) => {
                        println!("Failed to delete job[{}]: {} ", job_id, &err);
                        queue.fail_job(job_id).await
                    }
                };

                match res {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Failed to delete job[{}]: {} ", job_id, &err);
                    }
                }
            })
            .await;

        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

async fn handle_job(job: Job) -> Result<(), Error> {
    match job.message {
        message @ Message::SendSignInEmail { .. } => println!("Handling job: {:#?}", message),
        message @ Message::DeleteOldUserData => println!("Handling job: {:#?}", message),
        message @ Message::SendNewsLetterMessage { .. } => println!("Handling job: {:#?}", message),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub message: Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    SendSignInEmail {
        email: String,
        name: String,
        code: String,
    },
    DeleteOldUserData,
    SendNewsLetterMessage {
        message_id: Uuid,
    },
}

mod queue {
    use std::fmt::Debug;

    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::Message;

    #[async_trait::async_trait]
    pub trait Queue: Send + Sync + Debug {
        async fn push(
            &self,
            job: Message,
            scheduled_for: Option<chrono::DateTime<chrono::Utc>>,
        ) -> Result<(), crate::Error>;
        async fn pull(&self, number_of_jobs: u32) -> Result<Vec<Job>, crate::Error>;
        async fn delete_job(&self, job_id: Uuid) -> Result<(), crate::Error>;
        async fn fail_job(&self, job_id: Uuid) -> Result<(), crate::Error>;
        async fn clear(&self) -> Result<(), crate::Error>;
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Job {
        pub id: Uuid,
        pub message: Message,
    }
}

pub mod postgres_jobs {
    use sqlx::types::Json;
    use uuid::Uuid;

    use crate::{Job, Message};

    #[derive(sqlx::FromRow, Debug, Clone)]
    pub struct PostgresJob {
        id: Uuid,
        create_at: chrono::DateTime<chrono::Utc>,
        update_at: chrono::DateTime<chrono::Utc>,

        scheduled_for: Option<chrono::DateTime<chrono::Utc>>,
        failed_attempts: u32,
        status: PostgresJobStatus,
        message: Json<Message>,
    }

    #[repr(i32)]
    #[derive(Debug, Clone, sqlx::Type, PartialEq)]
    pub enum PostgresJobStatus {
        Queued,
        Running,
        Failed,
    }

    impl From<PostgresJob> for Job {
        fn from(value: PostgresJob) -> Self {
            Self {
                id: value.id,
                message: value.message.0,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PostgresQueue {
    pub db: DB,
    pub max_attempts: u32,
}
impl PostgresQueue {
    pub fn new(db: DB) -> PostgresQueue {
        PostgresQueue {
            db,
            max_attempts: 5,
        }
    }
}

#[async_trait::async_trait]
impl Queue for PostgresQueue {
    async fn push(
        &self,
        job: Message,
        date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), crate::Error> {
        let scheduled_for = date.unwrap_or(chrono::Utc::now());
        let failed_attempts: i32 = 0;
        let message = Json(job);
        let status = postgres_jobs::PostgresJobStatus::Queued;
        let now = chrono::Utc::now();
        let job_id: Uuid = Ulid::new().into();
        let query = "INSERT INTO queue
            (id, created_at, updated_at, scheduled_for, failed_attempts, status, message)
            VALUES ($1, $2, $3, $4, $5, $6, $7)";

        sqlx::query(query)
            .bind(job_id)
            .bind(now)
            .bind(now)
            .bind(scheduled_for)
            .bind(failed_attempts)
            .bind(status)
            .bind(message)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn pull(&self, number_of_jobs: u32) -> Result<Vec<Job>, Error> {
        let now = chrono::Utc::now();
        let query = "UPDATE queue
            SET status = $1, updated_at = $2
            WHERE id IN (
                SELECT id
                FROM queue
                WHERE status = $3 AND scheduled_for <= $4 AND failed_attempts < $5
                ORDER BY scheduled_for
                FOR UPDATE SKIP LOCKED
                LIMIT $6
            ) RETURNING *";
        let jobs: Vec<PostgresJob> = sqlx::query_as::<_, PostgresJob>(query)
            .bind(postgres_jobs::PostgresJobStatus::Running)
            .bind(now)
            .bind(postgres_jobs::PostgresJobStatus::Queued)
            .bind(now)
            .bind(self.max_attempts)
            .bind(number_of_jobs)
            .fetch_all(&self.db)
            .await?;

        Ok(jobs.into_iter().map(|job| job.into()).collect())
    }

    async fn delete_job(&self, job_id: Uuid) -> Result<(), Error> {
        let query = "DELETE FROM queue WHERE id = $1";

        sqlx::query(query).bind(job_id).execute(&self.db).await?;
        Ok(())
    }

    async fn fail_job(&self, job_id: Uuid) -> Result<(), Error> {
        let now = chrono::Utc::now();
        let query = "UPDATE queue
            SET status = $1, updated_at = $2, failed_attempts = failed_attempts + 1
            WHERE id = $3";

        sqlx::query(query)
            .bind(PostgresJobStatus::Queued)
            .bind(now)
            .bind(job_id)
            .execute(&self.db)
            .await?;
    }

    async fn clear(&self) -> Result<(), Error> {
        let query = "DELETE FROM queue";
        sqlx::query(query).execute(&self.db).await?;
        Ok(())
    }
}
