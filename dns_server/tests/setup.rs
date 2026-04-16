use std::sync::Arc;

use axum::{Router, routing::{get, post}};
use dns_app::app_state::{AppState, SharedAppState};
use dns_server::{add_dns_record_handler::add_dns_record_handler, list_dns_records_for_hostname_handler::list_dns_records_for_host_name_handler, resolve_hostname_handler::resolve_hostname_handler, delete_dns_record_handler::delete_dns_record_handler};
use tokio::sync::RwLock;



pub fn get_app_router() -> Router {
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

    app
}