use axum::{Extension, Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::database::Database;

#[derive(Serialize, Deserialize, Debug)]
struct TransactionRequest {
    pub chain_id: String,
    pub tx_type: String,
    pub to: String,
    pub from: String,
    pub amount: u32
}


pub fn transaction_routes() -> Router {
    Router::new().route("/user/create/transaction", get(|Extension(db): Extension<Arc<Database>>, Json(payload): Json<TransactionRequest>| async move {

    }))
}
