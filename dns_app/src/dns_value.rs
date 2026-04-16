use dns_dto::{dns_record_dto::DnsRecordDto, resolve_hostname_request_dto::PointingRecordDto};
use dns_types::{
    hostname::{Hostname},
    ipv4_address::IpV4Address,
};
use std::{collections::BTreeMap, default::Default, mem};

pub struct AliasEntry {
    pub expiry_time: i64,
    pub alias: Hostname,
}

#[derive(Default)]
pub enum DnsValue {
    Alias(AliasEntry),
    Resolved(BTreeMap<IpV4Address, i64>),
    #[default]
    Expired,
}

impl DnsValue {
    pub fn dns_value_dto_to_dns_value(dto: DnsRecordDto, expiry_time: i64) -> DnsValue {
        match dto {
            DnsRecordDto::CnameRecord(hostname) => DnsValue::Alias(AliasEntry {
                expiry_time,
                alias: hostname,
            }),
            DnsRecordDto::ARecord(ip) => {
                let map = BTreeMap::from([(ip, expiry_time)]);
                DnsValue::Resolved(map)
            }
        }
    }

    pub fn dns_value_to_pointing_record_dto(&self) -> Option<PointingRecordDto> {
        match &self {
            DnsValue::Alias(alias_entry) => {
                Some(PointingRecordDto::CnameRecord(alias_entry.alias.clone()))
            }
            DnsValue::Resolved(_) => Some(PointingRecordDto::ARecord),
            DnsValue::Expired => None,
        }
    }

    pub fn fold_value(&mut self, new_value: DnsValue) -> bool {
        match (&self, new_value) {
            (DnsValue::Expired, new_value) => {
                let _ = mem::replace(self, new_value);
                true
            }
            (DnsValue::Alias(_), DnsValue::Alias(new_alias)) => {
                let _ = mem::replace(self, DnsValue::Alias(new_alias));
                true
            }
            (DnsValue::Resolved(old_set), DnsValue::Resolved(mut new_set)) => {
                let old_keys = old_set.keys();
                let missing_keys: Vec<IpV4Address> = old_keys
                    .filter(|key| !new_set.contains_key(key))
                    .cloned()
                    .collect();

                for missing_key in missing_keys {
                    new_set.insert(missing_key, old_set.get(&missing_key).unwrap().to_owned());
                }
                let _ = mem::replace(self, DnsValue::Resolved(new_set));
                true
            }
            (_, _) => false,
        }
    }
}


mod tests{



    use std::str::FromStr;

    use super::*;

    #[test]
    pub fn test_fold_on_expired(){
        let mut expired = DnsValue::Expired;
       assert!(expired.fold_value(DnsValue::Alias(AliasEntry{
            expiry_time: 1,
            alias: Hostname::from_str("www.a.com").unwrap(),
        })));
        assert!(matches!(expired, DnsValue::Alias(_)));

        let mut expired = DnsValue::Expired;
        assert!(expired.fold_value(DnsValue::Resolved(BTreeMap::new())));
        assert!(matches!(expired, DnsValue::Resolved(_)));
    }

    #[test]
    pub fn test_fold_on_alias(){
        let mut dns_value = DnsValue::Alias(AliasEntry{
            expiry_time: 1,
            alias: Hostname::from_str("www.a.com").unwrap(),
        });

        assert!(!dns_value.fold_value(DnsValue::Expired));
        assert!(matches!(dns_value, DnsValue::Alias(_)));

        let mut dns_value = DnsValue::Alias(AliasEntry{
            expiry_time: 1,
            alias: Hostname::from_str("www.a.com").unwrap(),
        });

        assert!(!dns_value.fold_value(DnsValue::Resolved(BTreeMap::new())));
        assert!(matches!(dns_value, DnsValue::Alias(_)));

        let new_alias = DnsValue::Alias(AliasEntry{
            expiry_time: 1,
            alias: Hostname::from_str("www.b.com").unwrap(),
        });

        assert!(dns_value.fold_value(new_alias));
        assert!(match dns_value{
            DnsValue::Alias(alias_entry) => alias_entry.alias == Hostname::from_str("www.b.com").unwrap(),
            _ => false,
        })
    }



}
