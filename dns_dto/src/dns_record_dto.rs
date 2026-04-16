use dns_types::{hostname::Hostname, ipv4_address::IpV4Address};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type", content = "value")]
pub enum DnsRecordDto {
    #[serde(rename = "A")]
    ARecord(IpV4Address),
    #[serde(rename = "CNAME")]
    CnameRecord(Hostname),
}

mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let hostname: Hostname = "www.a.com".parse().unwrap();

        let dns_record: DnsRecordDto = DnsRecordDto::CnameRecord(hostname);

        let generated_json = serde_json::to_string(&dns_record).unwrap();

        let exepcted_json = "{\"type\":\"CNAME\",\"value\":\"www.a.com\"}".to_string();

        assert_eq!(exepcted_json, generated_json);
    }
}
