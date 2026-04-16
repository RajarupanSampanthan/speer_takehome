use crate::path_parameters::HostnamePathParam;
use axum::{
    Json, extract::{Path, State}, http::StatusCode
};
use axum_macros::debug_handler;
use dns_app::app_state::{SharedAppState};
use dns_dto::resolve_hostname_request_dto::ResolveHostnameRequestDto;

#[debug_handler]
pub async fn resolve_hostname_handler(
    state: State<SharedAppState>,
    path_param: Path<HostnamePathParam>,
) -> (StatusCode, Json<Option<ResolveHostnameRequestDto>> ) {
    println!("Hostnname is {:?}", &path_param.hostname);

    let app_state = state.read().await;

    if let Some(response) = app_state.resolve_hostname(&path_param.hostname) {
        return (StatusCode::OK, Json(Some(response)));
    }
    (StatusCode::NOT_FOUND, Json(None))
}
