use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    response::Html,
};
use axum_macros::debug_handler;
use dns_app::app_state::SharedAppState;
use dns_dto::add_dns_record_request_dto::AddDnsRecordRequestDto;

use crate::dns_error::DnsError;

#[debug_handler]
pub async fn add_dns_record_handler(
    state: State<SharedAppState>,
    add_dns_request: Result<Json<AddDnsRecordRequestDto>, JsonRejection>,
) -> () {
    if add_dns_request.is_err() {
        println!("Failed to parse Add Dns Request");
        return;
    }

    let Json(request_data) = add_dns_request.unwrap();

    let read_lock = state.read();
}
