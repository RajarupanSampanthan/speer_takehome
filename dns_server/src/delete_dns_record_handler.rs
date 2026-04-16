use axum::{
    extract::{Path, Query, State, rejection::QueryRejection},
    http::StatusCode,
    response::Html,
};
use axum_macros::debug_handler;
use dns_app::app_state::SharedAppState;
use dns_dto::delete_dns_record_request_dto::DeleteDnsRecordRequestDto;
use log::info;

use crate::path_parameters::HostnamePathParam;

#[debug_handler]
pub(crate) async fn delete_dns_record_handler(
    state: State<SharedAppState>,
    path_param: Path<HostnamePathParam>,
    query_parameters: Result<Query<DeleteDnsRecordRequestDto>, QueryRejection>,
) -> StatusCode {
    if let Ok(Query(query_data)) = query_parameters {
        println!("Query data : {:?}", query_data.0);
        StatusCode::ACCEPTED
    } else {
        StatusCode::BAD_REQUEST
    }
}
