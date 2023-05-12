/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use tonic::transport::Channel;

use crate::ledger_id::RefLedgerId;
use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::transaction::{
    AnyTransactionData,
    ChunkInfo,
    ToSchedulableTransactionDataProtobuf,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::{
    AccountId,
    BoxGrpcFuture,
    Error,
    TokenId,
    Transaction,
    ValidateChecksums,
};

/// Wipes the provided amount of tokens from the specified Account. Must be signed by the Token's
/// Wipe key.
///
/// On success, tokens are removed from the account and the total supply of the token is decreased by
/// the wiped amount.
///
/// The amount provided is in the lowest denomination possible. Example:
/// Token A has 2 decimals. In order to wipe 100 tokens from account, one must provide amount of 10000.
/// In order to wipe 100.55 tokens, one must provide amount of 10055.
///
/// - If the provided account is not found, the transaction will resolve to `INVALID_ACCOUNT_ID`.
/// - If the provided account has been deleted, the transaction will resolve to `ACCOUNT_DELETED`.
/// - If the provided token is not found, the transaction will resolve to `INVALID_TOKEN_ID`.
/// - If the provided token has been deleted, the transaction will resolve to `TOKEN_WAS_DELETED`.
/// - If an Association between the provided token and account is not found, the transaction will
/// resolve to `TOKEN_NOT_ASSOCIATED_TO_ACCOUNT`.
/// - If Wipe Key is not present in the Token, transaction results in `TOKEN_HAS_NO_WIPE_KEY`.
/// - If the provided account is the Token's Treasury Account, transaction results in
/// `CANNOT_WIPE_TOKEN_TREASURY_ACCOUNT`
/// - If both amount and serialNumbers get filled, a `INVALID_TRANSACTION_BODY` response code will be
/// returned.
/// - If neither the amount nor the serialNumbers get filled, a `INVALID_WIPING_AMOUNT` response code
/// will be returned.
/// - If the serialNumbers list contains a non-positive integer as a serial number, a `INVALID_NFT_ID`
/// response code will be returned.
/// - If the serialNumbers' list count is greater than the batch size limit global dynamic property, a
/// `BATCH_SIZE_LIMIT_EXCEEDED` response code will be returned.
///
pub type TokenWipeTransaction = Transaction<TokenWipeTransactionData>;

#[derive(Debug, Clone, Default)]
pub struct TokenWipeTransactionData {
    /// The account to be wiped.
    account_id: Option<AccountId>,

    /// The token for which the account will be wiped.
    token_id: Option<TokenId>,

    // TODO change type of `amount` from `Option<u64>` to `u64`
    /// The amount of a fungible token to wipe from the specified account.
    amount: Option<u64>,

    /// The serial numbers of a non-fungible token to wipe from the specified account.
    serials: Vec<u64>,
}

impl TokenWipeTransaction {
    /// Returns the account to be wiped.
    #[must_use]
    pub fn get_account_id(&self) -> Option<AccountId> {
        self.data().account_id
    }

    /// Sets the account to be wiped.
    pub fn account_id(&mut self, account_id: AccountId) -> &mut Self {
        self.data_mut().account_id = Some(account_id);
        self
    }

    /// Returns the token for which the account will be wiped.
    #[must_use]
    pub fn get_token_id(&self) -> Option<TokenId> {
        self.data().token_id
    }

    /// Sets the token for which the account will be wiped.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.data_mut().token_id = Some(token_id.into());
        self
    }

    /// Returns the amount of a fungible token to wipe from the specified account.
    #[must_use]
    pub fn get_amount(&self) -> Option<u64> {
        self.data().amount
    }

    // TODO remove `impl Into<_>`
    /// Sets the amount of a fungible token to wipe from the specified account.
    pub fn amount(&mut self, amount: impl Into<u64>) -> &mut Self {
        self.data_mut().amount = Some(amount.into());
        self
    }

    /// Returns the serial numbers of a non-fungible token to wipe from the specified account.
    #[must_use]
    pub fn get_serials(&self) -> &[u64] {
        &self.data().serials
    }

    /// Sets the serial numbers of a non-fungible token to wipe from the specified account.
    pub fn serials(&mut self, serials: impl IntoIterator<Item = u64>) -> &mut Self {
        self.data_mut().serials = serials.into_iter().collect();
        self
    }
}

impl TransactionData for TokenWipeTransactionData {}

impl TransactionExecute for TokenWipeTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { TokenServiceClient::new(channel).wipe_token_account(request).await })
    }
}

impl ValidateChecksums for TokenWipeTransactionData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        self.account_id.validate_checksums(ledger_id)?;
        self.token_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for TokenWipeTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::TokenWipe(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for TokenWipeTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::TokenWipe(self.to_protobuf())
    }
}

impl From<TokenWipeTransactionData> for AnyTransactionData {
    fn from(transaction: TokenWipeTransactionData) -> Self {
        Self::TokenWipe(transaction)
    }
}

impl FromProtobuf<services::TokenWipeAccountTransactionBody> for TokenWipeTransactionData {
    fn from_protobuf(pb: services::TokenWipeAccountTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            account_id: Option::from_protobuf(pb.account)?,
            token_id: Option::from_protobuf(pb.token)?,
            amount: Some(pb.amount),
            serials: pb.serial_numbers.into_iter().map(|it| it as u64).collect(),
        })
    }
}
impl ToProtobuf for TokenWipeTransactionData {
    type Protobuf = services::TokenWipeAccountTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::TokenWipeAccountTransactionBody {
            token: self.token_id.to_protobuf(),
            account: self.account_id.to_protobuf(),
            amount: self.amount.unwrap_or_default(),
            serial_numbers: self.serials.iter().map(|num| *num as i64).collect(),
        }
    }
}