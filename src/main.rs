use axum::{extract::State, routing::get, Router};
use shuttle_secrets::SecretStore;
use std::sync::Arc;

async fn hello_world(State(state): State<Arc<AppState>>) -> String {
    state.secret.to_string()
}

#[derive(Clone)]
struct AppState {
    secret: String,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {

    let Some(secret) = secret_store.get("MY_JWT_TOKEN") else {
       panic!("Secret Not find")
    };

    let state = Arc::new(AppState {
        secret,
    });
    let test = state.clone();

    
    let router = Router::new().route("/", get(hello_world)).with_state(test);

    Ok(router.into())
}