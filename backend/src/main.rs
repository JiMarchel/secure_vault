use backend::infra::{app::create_app, db::init_db, setup::init_app_state};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower_sessions::{Expiry, SessionManagerLayer, cookie::{SameSite, time::Duration}};
use tower_sessions_sqlx_store::PostgresStore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let app_state = init_app_state().await?;

    let pool = init_db().await?;

    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await?;

    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(Duration::hours(24)))
        .with_secure(false)
        .with_name("auth_session")
        .with_http_only(true)
        .with_path("/")
        .with_same_site(SameSite::Lax);

    let app = create_app(app_state, session_layer);

    let listener = TcpListener::bind("localhost:8000").await?;
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
