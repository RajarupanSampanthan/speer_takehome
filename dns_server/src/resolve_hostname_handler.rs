use crate::path_parameters::HostnamePathParam;
use axum::{
    extract::{Path, State},
    response::Html,
};
use axum_macros::debug_handler;
use dns_app::app_state::{self, SharedAppState};

#[debug_handler]
pub async fn resolve_hostname_handler(
    state: State<SharedAppState>,
    path_param: Path<HostnamePathParam>,
) -> () {
    println!("Hostnname is {:?}", path_param.hostname);
}
