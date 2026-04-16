use chrono::Utc;
use dns_dto::{
    add_dns_record_response_dto::AddDnsRecordResponseDto, dns_record_dto::DnsRecordDto,
    dns_record_list_responsedto::DnsRecordListResponseDto,
    resolve_hostname_request_dto::ResolveHostnameRequestDto,
};
use dns_types::{
    hostname::{Hostname},
    ipv4_address::IpV4Address,
};
use std::{
    collections::HashMap,

    sync::{Arc},
    time::Duration,
};
use tokio::sync::RwLock;

use crate::dns_value::{DnsValue};

pub type SharedAppState = Arc<RwLock<AppState>>;

pub struct AppState {
    hostname_to_dns_value: HashMap<Hostname, DnsValue>,
}

impl AppState {
    pub fn load_app_state() -> AppState {
        AppState {
            hostname_to_dns_value: HashMap::new(),
        }
    }

    pub fn add_dns_record(
        &mut self,
        hostname: Hostname,
        dto: DnsRecordDto,
    ) -> Option<AddDnsRecordResponseDto> {
        let expiry_time = Utc::now() + Duration::from_secs(60);
        let value_to_fold =
            DnsValue::dns_value_dto_to_dns_value(dto.clone(), expiry_time.timestamp());

        //Make sure CNAME cycles are not possible
        if let DnsValue::Alias(x) = &value_to_fold
            && self.check_if_hostname_to_target_is_possible(&x.alias, &hostname)
        {
            return None;
        }

        //Update value if it satisifies typ[e constraints
        let entry = self.hostname_to_dns_value.entry(hostname.clone());
        let existing_value = entry.or_insert_with(|| DnsValue::Expired);

        if !existing_value.fold_value(value_to_fold) {
            return None;
        }

        Some(AddDnsRecordResponseDto {
            hostname,
            dns_record_dto: dto,
            created_at: expiry_time.timestamp(),
        })
    }

    // This will set a tombstone on exisitng keys to mark them as 'deleted'. We can have a background job cleanup expired values
    pub fn delete_dns_record(&mut self, hostname: Hostname, dto: DnsRecordDto) -> bool {
        match (dto, self.hostname_to_dns_value.get_mut(&hostname)) {
            (DnsRecordDto::CnameRecord(given_alias), Some(DnsValue::Alias(existing_alias))) => {
                if given_alias != existing_alias.alias {
                    return false;
                }
                self.hostname_to_dns_value
                    .insert(hostname, DnsValue::Expired);
            }
            (DnsRecordDto::ARecord(single_ip), Some(DnsValue::Resolved(records))) => {
                if !records.contains_key(&single_ip) {
                    return false;
                }

                records.remove(&single_ip);

                if records.is_empty() {
                    self.hostname_to_dns_value
                        .insert(hostname, DnsValue::Expired);
                }
            }
            (_, None) => return false,
            _ => return false,
        }

        true
    }

    pub fn list_dns_records(&self, hostname: Hostname) -> Option<DnsRecordListResponseDto> {
        let optional_records = self
            .hostname_to_dns_value
            .get(&hostname)
            .map(|value| match value {
                DnsValue::Alias(alias_entry) => {
                    vec![DnsRecordDto::CnameRecord(alias_entry.alias.clone())]
                }
                DnsValue::Resolved(ip_map) => {
                    ip_map.keys().map(|ip| DnsRecordDto::ARecord(*ip)).collect()
                }
                DnsValue::Expired => vec![],
            })
            .map(|records| DnsRecordListResponseDto { hostname, records });

        // Return none if there are records is empty
        if let Some(DnsRecordListResponseDto {
            hostname: _,
            records,
        }) = &optional_records
            && records.is_empty()
        {
            return Option::None;
        }

        optional_records
    }

    pub fn resolve_hostname(&self, hostname: &Hostname) -> Option<ResolveHostnameRequestDto> {
        let not_alias_hostname = self.get_hostname_that_is_not_alias(hostname);

        let resolved_ips: Vec<IpV4Address> = not_alias_hostname
            .map(|name| self.hostname_to_dns_value.get(name))
            .and_then(|value| match value {
                Some(DnsValue::Resolved(x)) => Some(x.keys().cloned().collect()),
                _ => None,
            })?;

        let next_dns_record = self
            .hostname_to_dns_value
            .get(hostname)
            .and_then(|x| x.dns_value_to_pointing_record_dto())?;

        Some(ResolveHostnameRequestDto {
            hostname: hostname.clone(),
            resolved_ips,
            pointing_record: next_dns_record,
        })
    }

    fn check_if_hostname_to_target_is_possible(
        &mut self,
        start: &Hostname,
        target: &Hostname,
    ) -> bool {
        let mut current_hostname = start;

        while let Some(DnsValue::Alias(alias_entry)) =
            self.hostname_to_dns_value.get(current_hostname)
        {
            if alias_entry.alias == *target {
                return true;
            }

            current_hostname = &alias_entry.alias;
        }

        false
    }

    fn get_hostname_that_is_not_alias<'handler_scope>(
        &'handler_scope self,
        start: &'handler_scope Hostname,
    ) -> Option<&'handler_scope Hostname> {
        let mut current_hostname = start;

        while let Some(DnsValue::Alias(alias_entry)) =
            self.hostname_to_dns_value.get(current_hostname)
            && start != &alias_entry.alias
        {
            current_hostname = &alias_entry.alias;
        }

        match self.hostname_to_dns_value.get(current_hostname) {
            Some(DnsValue::Resolved(_)) => Some(current_hostname),
            _ => Option::None,
        }
    }
}

mod unit_tests {
    use std::str::FromStr;

    use super::*;
    use dns_types::hostname::Hostname;

    #[test]
    fn test_stop_cname_dns_record_add() {
        let mut app_state = AppState::load_app_state();

        let hostname_a = Hostname::from_str("www.a.com").unwrap();
        let hostname_b = Hostname::from_str("www.b.com").unwrap();

        assert!(app_state
            .add_dns_record(hostname_a.clone(), DnsRecordDto::CnameRecord(hostname_b.clone()))
            .is_some());

        // This should fail as it creates a cycle
        assert!(app_state
            .add_dns_record(hostname_b.clone(), DnsRecordDto::CnameRecord(hostname_a.clone()))
            .is_none());
    }

    #[test]
    fn test_detect_cname_reachability() {
        let mut app_state = AppState::load_app_state();

        let hostname_a = Hostname::from_str("www.a.com").unwrap();
        let hostname_b = Hostname::from_str("www.b.com").unwrap();
        let hostname_c = Hostname::from_str("www.c.com").unwrap();

        assert!(app_state
            .add_dns_record(hostname_a.clone(), DnsRecordDto::CnameRecord(hostname_b.clone()))
            .is_some());

        assert!(app_state
            .add_dns_record(hostname_b.clone(), DnsRecordDto::CnameRecord(hostname_c.clone()))
            .is_some());

        assert!(app_state.check_if_hostname_to_target_is_possible(&hostname_a, &hostname_c));

        assert!(app_state.get_hostname_that_is_not_alias(&hostname_a).is_none());
    }

    #[test]
    fn test_ends_with_arecord() {
        let mut app_state = AppState::load_app_state();

        let hostname_a = Hostname::from_str("www.a.com").unwrap();
        let hostname_b = Hostname::from_str("www.b.com").unwrap();
        let hostname_c = Hostname::from_str("www.c.com").unwrap();

        assert!(app_state
            .add_dns_record(hostname_a.clone(), DnsRecordDto::CnameRecord(hostname_b.clone()))
            .is_some());

        let ip = IpV4Address::from_str("1.1.1.1").unwrap();

        assert!(app_state
            .add_dns_record(hostname_b.clone(), DnsRecordDto::ARecord(ip.clone()))
            .is_some());

        assert!(!app_state.check_if_hostname_to_target_is_possible(&hostname_a, &hostname_c));

        let last_host = app_state.get_hostname_that_is_not_alias(&hostname_a);
        assert!(last_host.is_some());
        assert_eq!(&hostname_b, last_host.unwrap());

        let record = app_state.hostname_to_dns_value.get(&hostname_b).unwrap();

        assert!(matches!(record, DnsValue::Resolved(x)));
    }
}
