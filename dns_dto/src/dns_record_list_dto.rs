use crate::dns_record_dto::DnsRecordDto;
use dns_types::hostname::Hostname;

pub struct DnsRecordListDto {
    pub hostname: Hostname,

    pub records: Vec<DnsRecordDto>,
}
