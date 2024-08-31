use std::env;


struct Config {
    pub db_url: String,
}

impl Config {
    fn from_env() -> Result<Self, String> {
        let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Ok(Self { db_url })
    }
}

pub trait ModelsRepository {
    fn create_author(&self, req: &CreateAuthorRequest) -> anyhow::Result<CreateAuthorResponse>;
}

pub trait AuthorRepository {
    fn create(&self, req: &CreateAuthorRequest) -> anyhow::Result<CreateAuthorResponse>;
}

#[derive(Clone, Debug)]
pub struct Author {
    id: UUID,
    name: AuthorName,
}

impl Author {
    pub fn new(id: UUID, name: AuthorName) -> Self {
            Self { id, name }
    }
    pub fn id(&self) -> &UUID {
        &self.id
    }

    pub fn name(&self) -> &AuthorName {
        &self.name
    }
}

#[derive(Clone, Debug)]
pub struct AuthorName(String);

#[derive(Clone, Debug, Error)]
#[error("author name cannot be empty")]
pub struct AuthorNameEmptyError;

impl AuthorName {
    pub fn new(raw: &str) -> Result<Self, AuthorNameEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(AuthorNameEmptyError)
        } else { 
            Ok(Self(trimmed.to_string()))
        }
    }
}

#[derive(Clone, Debug)]
pub struct CreateAuthorRequest {
    name: AuthorName,
}

impl CreateAuthorRequest {}

#[derive(Debug, Error)]
pub enum CreateAuthorError {
    #[error("author with name {name} already exists")]
    Duplicate { name: AuthorName },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env();

    // tracing_subscriber::fmt::init();
    // let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
    //     |request: &axum::extract::Requext<_>| {
    //         let uri = request.uri().to_string();
    //         tracing::info_span!("http_request", method = ?request.method(), uri)
    //     },
    // );
    //
    // let sqlite_pool = SqlitePool::connect_with(
    //     SqliteConnectOptions::from_str(&config.database_url)
    //         .with_context(|| format!("invalid database path {}", &config.database_url))?
    //         .pragma("foreign_keys", "ON"),
    // )
    // .await
    // .with_context(|| format!("failed to open database at {}", &config.database_url));
    //
    // let app_state = AppState {
    //     sqlite: Arc::new(sqlite_pool),
    // };
    // let router = axum::Router::new()
    //     .route("/authors", post(create_author))
    //     .layer(trace_layer)
    //     .with_state(app_state);
    // let listener = net::TcpListener::bind(format!("0.0.0.0:{}", &config.server_port))
    //     .await
    //     .with_context(|| format!("failed to listen on {}", &config.server_port))?;
    //
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    // axum::serve(listener, router)
    //     .await
    //     .context("received error from running server")?;
    //
    Ok(())
}
