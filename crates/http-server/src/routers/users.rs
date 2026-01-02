use axum::{Router, routing::get};

use crate::{extractors::state::AppState, handlers::users::me};

pub fn routes() -> Router<AppState> {
    Router::new().route("/me", get(me::handler))
}
