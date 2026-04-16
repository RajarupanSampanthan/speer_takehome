use dns_types::hostname::Hostname;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HostnamePathParam {
    pub hostname: Hostname,
}
