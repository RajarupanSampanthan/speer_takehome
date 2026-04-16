use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
};
use axum_macros::debug_handler;
use dns_app::app_state::SharedAppState;
use dns_dto::{
    add_dns_record_request_dto::AddDnsRecordRequestDto,
    add_dns_record_response_dto::AddDnsRecordResponseDto,
};

#[debug_handler]
pub async fn add_dns_record_handler(
    state: State<SharedAppState>,
    add_dns_request: Result<Json<AddDnsRecordRequestDto>, JsonRejection>,
) -> (StatusCode, Json<Option<AddDnsRecordResponseDto>>) {
    if add_dns_request.is_err() {
        println!("Failed to parse Add Dns Request");
        return (StatusCode::BAD_REQUEST, Json(None));
    }

    let Json(request_data) = add_dns_request.unwrap();

    let mut app_state = state.write().await;

    if let Some(x) = app_state.add_dns_record(request_data.hostname, request_data.dns_record_dto) {
        return (StatusCode::OK, Json(Some(x)));
    }

    (StatusCode::BAD_REQUEST, Json(None))
}
