use dns_types::{hostname::Hostname, ipv4_address::IpV4Address};
use serde::Deserialize;

pub use serde::Serialize;

use crate::dns_record_dto::DnsRecordDto;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct DeleteDnsRecordRequestDto(pub DnsRecordDto);

mod tests {
    use std::any::Any;

    use axum::{extract::Query, http::Uri};

    use super::*;

    #[test]
    fn test_delete_request_with_cname_record() {
        let uri_1: Uri = "http://localhost:3000/api/dns/host_1?type=CNAME&value=www.a.com"
            .parse()
            .unwrap();

        let uri_2: Uri = "http://localhost:3000/api/dns/host_1?value=www.a.com&type=CNAME"
            .parse()
            .unwrap();

        for uri in [uri_1, uri_2] {
            let result: Query<DeleteDnsRecordRequestDto> = Query::try_from_uri(&uri).unwrap();
            let DeleteDnsRecordRequestDto(dto) = result.0;

            let value: Hostname = "www.a.com".parse().unwrap();
            let expected_record = DnsRecordDto::CnameRecord(value);

            assert_eq!(expected_record, dto);
        }
    }

    #[test]
    fn test_delete_request_with_a_record() {
        let uri_1: Uri = "http://localhost:3000/api/dns/host_1?type=A&value=1.1.1.1"
            .parse()
            .unwrap();

        let uri_2: Uri = "http://localhost:3000/api/dns/host_1?value=1.1.1.1&type=A"
            .parse()
            .unwrap();

        for uri in [uri_1, uri_2] {
            let result: Query<DeleteDnsRecordRequestDto> = Query::try_from_uri(&uri).unwrap();
            let DeleteDnsRecordRequestDto(dto) = result.0;

            let value: IpV4Address = "1.1.1.1".parse().unwrap();
            let expected_record = DnsRecordDto::ARecord(value);

            assert_eq!(expected_record, dto);
        }
    }
}
