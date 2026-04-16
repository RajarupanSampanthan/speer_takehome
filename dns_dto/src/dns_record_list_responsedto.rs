use crate::dns_record_dto::DnsRecordDto;
use dns_types::hostname::Hostname;
use serde::Serialize;

#[derive(Serialize)]
pub struct DnsRecordListResponseDto {
    pub hostname: Hostname,

    pub records: Vec<DnsRecordDto>,
}
