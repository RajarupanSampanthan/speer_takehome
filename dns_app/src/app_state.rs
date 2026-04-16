use dns_dto::{
    dns_record_dto::DnsRecordDto, dns_record_list_dto::DnsRecordListDto,
    resolve_hostname_request_dto::ResolveHostnameRequestDto,
};
use dns_types::{hostname::Hostname, ipv4_address::IpV4Address};
use std::{
    collections::HashMap,
    mem,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};
use tokio::sync::RwLock;

use crate::live_dns_entry::LiveDnsEntry;

pub type SharedAppState = Arc<RwLock<AppState>>;

pub struct AppState {
    pub hostname_to_dns_entry: HashMap<Hostname, LiveDnsEntry>,
}

impl AppState {
    pub fn load_app_state() -> AppState {
        AppState {
            hostname_to_dns_entry: HashMap::new(),
        }
    }

    pub fn add_dns_record(&mut self, hostname: Hostname, dto: DnsRecordDto) -> bool {
        todo!()
    }

    pub fn delete_dns_record(&mut self, dto: DnsRecordDto) -> bool {
        todo!()
    }

    pub fn list_dns_records(&self, hostname: Hostname) -> Option<DnsRecordListDto> {
        todo!()
    }

    pub fn resolve_hostname(&self, hostname: Hostname) -> Option<ResolveHostnameRequestDto> {
        todo!()
    }
}
