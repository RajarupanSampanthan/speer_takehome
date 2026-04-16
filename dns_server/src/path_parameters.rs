use serde::Deserialize;

#[derive(Deserialize)]
pub struct HostnamePathParam {
    pub hostname: String,
}
