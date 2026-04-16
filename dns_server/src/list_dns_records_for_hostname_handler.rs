use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_macros::debug_handler;
use dns_app::app_state::SharedAppState;
use dns_dto::dns_record_list_responsedto::DnsRecordListResponseDto;

use crate::path_parameters::HostnamePathParam;

#[debug_handler]
pub async fn list_dns_records_for_host_name_handler(
    state: State<SharedAppState>,
    path_param: Path<HostnamePathParam>,
) -> (StatusCode, Json<Option<DnsRecordListResponseDto>>) {
    let app_state = state.read().await;

    if let Some(response) = app_state.list_dns_records(path_param.0.hostname) {
        (StatusCode::OK, Json(Some(response)))
    } else {
        (StatusCode::NOT_FOUND, Json(None))
    }
}
