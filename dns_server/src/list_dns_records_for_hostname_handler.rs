use axum::{
    extract::{Path, State},
    response::Html,
};
use axum_macros::debug_handler;
use dns_app::app_state::SharedAppState;

use crate::path_parameters::HostnamePathParam;

#[debug_handler]
pub async fn list_dns_records_for_host_name_handler(
    state: State<SharedAppState>,
    path_param: Path<HostnamePathParam>,
) -> () {
    println!("Hostnname is {:?}", path_param.hostname);
}
