use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::file_service_client::FileServiceClient;
use hedera_proto::services::smart_contract_service_client::SmartContractServiceClient;
use serde_with::{
    serde_as,
    TimestampNanoSeconds,
};
use time::OffsetDateTime;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    ContractId,
    FileId,
    Transaction,
};

pub type SystemDeleteTransaction = Transaction<SystemDeleteTransactionData>;

/// Delete a file or smart contract - can only be done with a Hedera admin.
///
/// When it is deleted, it immediately disappears from the system as seen by the user,
/// but is still stored internally until the expiration time, at which time it
/// is truly and permanently deleted.
///
/// Until that time, it can be undeleted by the Hedera admin.
/// When a smart contract is deleted, the cryptocurrency account within it continues
/// to exist, and is not affected by the expiration time here.
///
#[serde_as]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SystemDeleteTransactionData {
    #[serde_as(as = "Option<TimestampNanoSeconds>")]
    pub expiration_time: Option<OffsetDateTime>,
    pub file_id: Option<FileId>,
    pub contract_id: Option<ContractId>,
}

impl SystemDeleteTransaction {
    /// Sets the contract ID which should be deleted.
    pub fn contract_id(&mut self, id: impl Into<ContractId>) -> &mut Self {
        self.body.data.file_id = None;
        self.body.data.contract_id = Some(id.into());
        self
    }

    /// Sets the file ID which should be deleted.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        self.body.data.contract_id = None;
        self.body.data.file_id = Some(id.into());
        self
    }

    /// Sets the timestamp at which the "deleted" file should
    /// truly be permanently deleted.
    pub fn expiration_time(&mut self, expiration_time: OffsetDateTime) -> &mut Self {
        self.body.data.expiration_time = Some(expiration_time);
        self
    }
}

#[async_trait]
impl TransactionExecute for SystemDeleteTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        if self.file_id.is_some() {
            FileServiceClient::new(channel).system_delete(request).await
        } else {
            SmartContractServiceClient::new(channel).system_delete(request).await
        }
    }
}

impl ToTransactionDataProtobuf for SystemDeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let expiration_time = self.expiration_time.map(Into::into);
        let contract_id = self.contract_id.as_ref().map(ContractId::to_protobuf);
        let file_id = self.file_id.as_ref().map(FileId::to_protobuf);

        let id = match (contract_id, file_id) {
            (Some(contract_id), _) => {
                Some(services::system_delete_transaction_body::Id::ContractId(contract_id))
            }

            (_, Some(file_id)) => {
                Some(services::system_delete_transaction_body::Id::FileId(file_id))
            }

            _ => None,
        };

        services::transaction_body::Data::SystemDelete(services::SystemDeleteTransactionBody {
            expiration_time,
            id,
        })
    }
}

impl From<SystemDeleteTransactionData> for AnyTransactionData {
    fn from(transaction: SystemDeleteTransactionData) -> Self {
        Self::SystemDelete(transaction)
    }
}