use axum::{
    extract::{Path, Query, State, rejection::QueryRejection},
    http::StatusCode,
};
use axum_macros::debug_handler;
use dns_app::app_state::SharedAppState;
use dns_dto::delete_dns_record_request_dto::DeleteDnsRecordRequestDto;

use crate::path_parameters::HostnamePathParam;

#[debug_handler]
pub async fn delete_dns_record_handler(
    state: State<SharedAppState>,
    path_param: Path<HostnamePathParam>,
    query_parameters: Result<Query<DeleteDnsRecordRequestDto>, QueryRejection>,
) -> StatusCode {
    if let Ok(Query(query_data)) = query_parameters {
        println!("Query data : {:?}", query_data.0);

        let mut app_state = state.write().await;

        if !app_state.delete_dns_record(path_param.0.hostname, query_data.0) {
            StatusCode::BAD_REQUEST
        } else {
            StatusCode::ACCEPTED
        }
    } else {
        StatusCode::BAD_REQUEST
    }
}
