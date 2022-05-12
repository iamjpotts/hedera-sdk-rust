use hedera_proto::services;

pub trait ToQueryProtobuf: Send + Sync {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query;
}