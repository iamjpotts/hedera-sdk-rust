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
use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::token::token_info::TokenInfo;
use crate::{
    BoxGrpcFuture,
    Error,
    Query,
    ToProtobuf,
    TokenId,
    ValidateChecksums,
};

/// Gets information about Token instance.
///
pub type TokenInfoQuery = Query<TokenInfoQueryData>;

#[derive(Default, Clone, Debug)]
pub struct TokenInfoQueryData {
    token_id: Option<TokenId>,
}

impl From<TokenInfoQueryData> for AnyQueryData {
    #[inline]
    fn from(data: TokenInfoQueryData) -> Self {
        Self::TokenInfo(data)
    }
}

impl TokenInfoQuery {
    /// Returns the token ID for which information is requested.
    #[must_use]
    pub fn get_token_id(&self) -> Option<TokenId> {
        self.data.token_id
    }

    /// Sets the token ID for which information is requested.
    pub fn token_id(&mut self, id: impl Into<TokenId>) -> &mut Self {
        self.data.token_id = Some(id.into());
        self
    }
}

impl ToQueryProtobuf for TokenInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let token_id = self.token_id.to_protobuf();

        services::Query {
            query: Some(services::query::Query::TokenGetInfo(services::TokenGetInfoQuery {
                header: Some(header),
                token: token_id,
            })),
        }
    }
}

impl QueryExecute for TokenInfoQueryData {
    type Response = TokenInfo;

    fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> BoxGrpcFuture<'_, services::Response> {
        Box::pin(async { TokenServiceClient::new(channel).get_token_info(request).await })
    }
}

impl ValidateChecksums for TokenInfoQueryData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        self.token_id.validate_checksums(ledger_id)
    }
}