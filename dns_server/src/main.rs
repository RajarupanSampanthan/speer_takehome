//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use std::sync::{Arc, Mutex};

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    response::Html,
    routing::{get, post},
};

use axum_macros::debug_handler;
use dns_app::app_state::{AppState, SharedAppState};
use tokio::sync::RwLock;

use crate::{
    add_dns_record_handler::add_dns_record_handler,
    delete_dns_record_handler::delete_dns_record_handler,
    list_dns_records_for_hostname_handler::list_dns_records_for_host_name_handler,
    path_parameters::HostnamePathParam, resolve_hostname_handler::resolve_hostname_handler,
};

mod add_dns_record_handler;
mod delete_dns_record_handler;
mod dns_error;
mod list_dns_records_for_hostname_handler;
mod path_parameters;
mod resolve_hostname_handler;

#[tokio::main]
async fn main() {
    let state: SharedAppState = Arc::new(RwLock::new(AppState::load_app_state()));

    // build our application with a route
    let app = Router::new()
        .route("/api/dns", post(add_dns_record_handler))
        .route(
            "/api/dns/{hostname}",
            get(resolve_hostname_handler).delete(delete_dns_record_handler),
        )
        .route(
            "/api/dns/{hostname}/records",
            get(list_dns_records_for_host_name_handler),
        )
        .with_state(state);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await;
}
