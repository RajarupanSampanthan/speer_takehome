use dns_types::{hostname::Hostname, ipv4_address::IpV4Address};
use std::time::SystemTime;

pub struct ResolvedEntry {
    pub expiry_time: SystemTime,
    pub ip: IpV4Address,
}

pub struct AliasEntry {
    pub expiry_time: SystemTime,
    pub alias: Hostname,
}

pub enum LiveDnsEntry {
    Alias(AliasEntry),
    Resolved(Vec<ResolvedEntry>),
}
