use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::consensus_service_client::ConsensusServiceClient;
use serde_with::skip_serializing_none;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountId, TopicId, Transaction, TransactionId};

/// Delete a topic.
///
/// No more transactions or queries on the topic will succeed.
///
/// If an `admin_key` is set, this transaction must be signed by that key.
/// If there is no `admin_key`, this transaction will fail `UNAUTHORIZED`.
///
pub type TopicDeleteTransaction = Transaction<TopicDeleteTransactionData>;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicDeleteTransactionData {
    /// The topic ID which is being deleted in this transaction.
    topic_id: Option<TopicId>,
}

impl TopicDeleteTransaction {
    /// Set the account ID which is being deleted.
    pub fn topic_id(&mut self, id: impl Into<TopicId>) -> &mut Self {
        self.body.data.topic_id = Some(id.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for TopicDeleteTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        ConsensusServiceClient::new(channel).delete_topic(request).await
    }
}

impl ToTransactionDataProtobuf for TopicDeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let topic_id = self.topic_id.as_ref().map(TopicId::to_protobuf);

        services::transaction_body::Data::ConsensusDeleteTopic(
            services::ConsensusDeleteTopicTransactionBody { topic_id },
        )
    }
}

impl From<TopicDeleteTransactionData> for AnyTransactionData {
    fn from(transaction: TopicDeleteTransactionData) -> Self {
        Self::TopicDelete(transaction)
    }
}