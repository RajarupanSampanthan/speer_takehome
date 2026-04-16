use dns_types::{hostname::Hostname, ipv4_address::IpV4Address};
use serde::{Deserialize, Serialize};

use crate::dns_record_dto::DnsRecordDto;

#[derive(Serialize, serde::Deserialize, Debug, PartialEq)]
#[serde(tag = "recordType", content = "pointsTo")]
pub enum PointingRecordDto {
    #[serde(rename = "A")]
    ARecord,
    #[serde(rename = "CNAME")]
    CnameRecord(Hostname),
}

#[derive(Serialize, Deserialize)]
pub struct ResolveHostnameRequestDto {
    pub hostname: Hostname,

    #[serde(rename = "resolvedIps")]
    pub resolved_ips: Vec<IpV4Address>,

    #[serde(flatten)]
    pub pointing_record: PointingRecordDto,
}
