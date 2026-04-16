use chrono::Utc;
use dns_types::hostname::Hostname;
use serde::{Deserialize, Serialize};

use crate::dns_record_dto::DnsRecordDto;

#[derive(Serialize, Deserialize)]
pub struct AddDnsRecordResponseDto {
    #[serde(flatten)]
    pub dns_record_dto: DnsRecordDto,

    pub hostname: Hostname,

    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

mod tests {

    use std::str::FromStr;

    use dns_types::ipv4_address::IpV4Address;

    use crate::dns_record_dto;

    use super::*;

    #[test]
    fn test_serialize_add_dns_response_with_a_record() {
        let hostname = Hostname::from_str("www.a.com").unwrap();

        let dns_record_dto: DnsRecordDto =
            DnsRecordDto::ARecord(IpV4Address::from_str("1.1.1.1").unwrap());

        let time_stamp = Utc::now().timestamp();

        let add_dns_record_request = AddDnsRecordResponseDto {
            hostname,
            dns_record_dto,
            created_at: time_stamp,
        };

        let generated_json = serde_json::to_string(&add_dns_record_request).unwrap();

        let exepcted_json =
            "{\"type\":\"A\",\"value\":\"1.1.1.1\",\"hostname\":\"www.a.com\",\"createdAt\":"
                .to_string() + &time_stamp.to_string() + "}";

        assert_eq!(exepcted_json, generated_json);
    }

    #[test]
    fn test_serialize_add_dns_response_with_cname_record() {
        let hostname = Hostname::from_str("www.a.com").unwrap();

        let dns_record_dto: DnsRecordDto =
            DnsRecordDto::CnameRecord(Hostname::from_str("www.x.com").unwrap());

        let time_stamp = Utc::now().timestamp();

        let add_dns_record_request = AddDnsRecordResponseDto {
            hostname,
            dns_record_dto,
            created_at: time_stamp,
        };

        let generated_json = serde_json::to_string(&add_dns_record_request).unwrap();

        let exepcted_json =
            "{\"type\":\"CNAME\",\"value\":\"www.x.com\",\"hostname\":\"www.a.com\",\"createdAt\":"
                .to_string() + &time_stamp.to_string() + "}";

        assert_eq!(exepcted_json, generated_json);
    }
}
