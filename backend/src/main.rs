use axum::{extract::State, http::StatusCode, routing::{get,post}, Json, Router};
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, net::SocketAddr};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[derive(Clone)]
struct AppState { 
    config: AppConfig,
    db: PgPool,
}

#[derive(Clone)]
struct AppConfig {
    app_name: String,
    host: String,
    port: u16,
    database_url: String,
}

impl AppConfig {
    fn from_env() -> Self {

        let app_name = env::var("APP_NAME")
            .unwrap_or_else(|_| "oquest-backend".to_string());

        let host = env::var("APP_HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = env::var("APP_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(3000);

        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self { app_name, host, port, database_url, }
    }

    fn socket_addr(&self) -> SocketAddr { // references instance
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("APP_HOST or APP_PORT should be valid")
    }
}

// Request/Response types /////////////////////////////////////////////////////////////
// serialize using Serde from struct to JSON
#[derive(Serialize)]
struct HealthResponse {
    ok: bool,
    app_name: String,
}

#[derive(Serialize)]
struct DbCheckResponse {
    ok: bool,
    db: &'static str,
}

#[derive(Serialize)]
struct ErrorResponse {
    ok: bool,
    error: String,
}

#[derive(Serialize)]
struct SyncIdentityResponse {
    user_id: i64,
    created: bool,
}

// Serde turns into this Rust struct
#[derive(Deserialize)]
struct SyncIdentityRequest {
    provider: String,
    provider_subject: String,
    email: Option<String>,
    display_name: Option<String>,
}

// Handlers ///////////////////////////////////////////////////////////////////
// route handler for health
async fn health(State(state) : State<AppState>) -> Json<HealthResponse>  { 
    Json(HealthResponse { 
        ok: true,
        app_name: state.config.app_name.clone(),
     })
}

// handler for db testing
async fn db_check(State(state): State<AppState>,) 
-> Result< Json<DbCheckResponse>, (StatusCode, Json<ErrorResponse>) > {
    let value: i32 = sqlx::query_scalar("SELECT 1")
        .fetch_one(&state.db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    ok: false,
                    error: format!("database check failed: {err}"),
                }),
            )
        })?;

    if value == 1 {
        Ok(Json(DbCheckResponse {
            ok: true,
            db: "reachable",
        }))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                ok: false,
                error: format!("unexpected db check value: {value}"),
            }),
        ))
    }
}

async fn sync_identity
    (State(state): State<AppState>,
    Json(payload): Json<SyncIdentityRequest>)
    -> Result<Json<SyncIdentityResponse>, (StatusCode, Json<ErrorResponse>)> {

    // queries and extracts first row (query_scalar), user existence check
    let existing_user_id: Option<i64> = sqlx::query_scalar(
        r#"
        SELECT user_id
        FROM user_identities
        WHERE provider = $1
        AND provider_subject = $2
        "#,
    )
    .bind(&payload.provider)
    .bind(&payload.provider_subject)
    .fetch_optional(&state.db)
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                ok: false,
                error: format!("failed to query existing identity: {err}"),
            }),
        )
    })?;

    // if user already exists in system, return it.
    if let Some(user_id) = existing_user_id {
        return Ok(Json(SyncIdentityResponse {
            user_id,
            created: false,
        }));
    }

    // user does not exist in user_identities table /////////////////////////
    let mut tx = state.db.begin().await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse{
                ok:false,
                error: format!("failed to being transaction: {err}"),
            }),
        )
    })?;

    // insertion of new user into tables
    let user_id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO users (display_name)
        VALUES ($1)
        RETURNING id
        "#,
        )
        .bind(&payload.display_name)
        // execute and retrieve id
        .fetch_one(&mut *tx) 
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    ok: false,
                    error: format!("failed to create user: {err}"),
                }),
            )
    })?;
    sqlx::query(
        r#"
        INSERT INTO user_identities (user_id, provider, provider_subject, email)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(user_id)
    .bind(&payload.provider)
    .bind(&payload.provider_subject)
    .bind(&payload.email)
    .execute(&mut *tx)
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                ok: false,
                error: format!("failed to create user identity: {err}"),
            }),
        )
    })?;

    tx.commit().await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                ok: false,
                error: format!("failed to commit transaction: {err}"),
            }),
        )
    })?;

    // successfully registered new user
    Ok(Json(SyncIdentityResponse {
        user_id,
        created: true,
    }))
    
}

// program entry //////////////////////////////////////////////////////////////
#[tokio::main]
async fn main() {
    // pulling environment variables from .env file
    dotenvy::dotenv().ok();

    // logging subscriber
    // seeing logs in terminal
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("backend=debug,tower_http=debug"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::from_env();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("failed to connect to PostgreSQL");

    // init persistent app state
    let state = AppState {
        config: config.clone(),
        db,
    };

    // app build routes
    let app = Router::new()
        .route("/health", get(health))
        .route("/db-check", get(db_check))
        .route("/dev/users/sync-identity", post(sync_identity))
        .layer(TraceLayer::new_for_http())
        .with_state(state); // provide state to handler

    // server listening socket, localhost for now
    let addr = config.socket_addr();

    // opens socket
    let listener = tokio::net::TcpListener::bind(addr).await
        .expect("shold be able to bind to TCP listener");

    tracing::info!("listening on http://{}", addr);

    // accept incoming connections
    axum::serve(listener, app).await
        .expect("server failed");
}

