use backend::infra::{app::create_app, setup::init_app_state};
use dotenvy::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let app_state = init_app_state().await?;

    let app = create_app(app_state.state, app_state.session_layer);

    let listener = TcpListener::bind("localhost:8000").await?;
    axum::serve(listener, app).await.unwrap();

    println!("Server running on http://localhost:8000");

    Ok(())
}
