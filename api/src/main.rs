mod routes;
mod config;

use axum::{Json};
use axum::{routing::get};
use sea_orm::DatabaseConnection;
use std::time::Duration;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{Span, error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::{OpenApi};
use utoipa_scalar::{Scalar, Servable};
use utoipa_axum::router::OpenApiRouter;
use routes::user;

use crate::config::AppConfig;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "user", description = "User Routes")
    ),
    info(
        title = "NusaBelajar Auth API",
        version = "0.1.0",
        description = "Simple API for authentication"
    )
)]
struct ApiDoc;

#[dotenvy::load(path = ".env", required = false)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_config = AppConfig::load();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("🚀 Starting API server...");

    let connection = match sea_orm::Database::connect(&app_config.database_url).await {
        Ok(conn) => {
            info!("✅ Connected to database");
            conn
        }
        Err(err) => {
            error!("❌ Failed to connect to database: {}", err);
            std::process::exit(1);
        }
    };

    let state = AppState { connection };

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .with_state(state)
        .nest("/user", user::router())
        .split_for_parts();

    let app = router
        .route("/", get(routes::root::route))
        .merge(Scalar::with_url("/scalar", api.clone()))
        .route("/scalar/openapi.json", get(Json(api)))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO))
                .on_failure(
                    |class: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                        error!(
                            ?class,
                            latency_ms = latency.as_millis(),
                            "⚠️ request failed"
                        );
                    },
                ),
        );

    let address = format!("0.0.0.0:{}", &app_config.server_port);

    info!("🌐 Listening on {}", address);

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
struct AppState {
    connection: DatabaseConnection,
}
