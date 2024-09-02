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

use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

use tonic::transport::Endpoint;

use crate::signer::AnySigner;
use crate::{
    AccountId,
    PrivateKey,
};

struct FromStrProxy<T>(T);

impl<'de, T: FromStr> serde::Deserialize<'de> for FromStrProxy<T>
where
    T::Err: std::fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        String::deserialize(deserializer)
            .and_then(|it| it.parse().map_err(D::Error::custom))
            .map(Self)
    }
}

#[derive(serde_derive::Deserialize)]
pub(super) struct Operator {
    account_id: FromStrProxy<AccountId>,
    private_key: FromStrProxy<PrivateKey>,
}

impl From<Operator> for super::Operator {
    fn from(value: Operator) -> Self {
        Self { account_id: value.account_id.0, signer: AnySigner::PrivateKey(value.private_key.0) }
    }
}

#[derive(serde_derive::Deserialize)]
#[serde(untagged)]
pub(super) enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum NetworkName {
    Mainnet,
    Testnet,
    Previewnet,
}

#[derive(serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ClientConfigInner {
    operator: Option<Operator>,
    network: Either<HashMap<String, FromStrProxy<AccountId>>, NetworkName>,
    mirror_network: Option<Either<Vec<String>, NetworkName>>,
}

impl From<ClientConfigInner> for ClientConfig {
    fn from(value: ClientConfigInner) -> Self {
        Self {
            operator: value.operator.map(Into::into),
            network: match value.network {
                Either::Left(it) => Either::Left(it.into_iter().map(|(k, v)| (k, v.0)).collect()),
                Either::Right(it) => Either::Right(it),
            },
            mirror_network: value.mirror_network,
        }
    }
}

pub(super) struct ClientConfig {
    pub(super) operator: Option<super::Operator>,
    pub(super) network: Either<HashMap<String, AccountId>, NetworkName>,
    pub(super) mirror_network: Option<Either<Vec<String>, NetworkName>>,
}

/// gRPC channel connection configuration
pub struct EndpointConfig {
    pub connect_timeout: Option<Duration>,
    pub http2_keep_alive_interval: Option<Duration>,
    pub http2_keep_alive_timeout: Option<Duration>,
    pub http2_keep_alive_while_idle: Option<bool>,
    pub tcp_keepalive: Option<Duration>,
}

impl EndpointConfig {
    /// Provide a default configuration
    pub fn new() -> Self {
        Self {
            connect_timeout: Some(Duration::from_secs(10)),
            http2_keep_alive_interval: None,
            http2_keep_alive_timeout: Some(Duration::from_secs(10)),
            http2_keep_alive_while_idle: Some(true),
            tcp_keepalive: Some(Duration::from_secs(10)),
        }
    }
}

impl EndpointConfig {
    pub(crate) fn apply(&self, endpoint: Endpoint) -> Endpoint {
        let endpoint = if let Some(value) = self.connect_timeout {
            endpoint.connect_timeout(value)
        } else {
            endpoint
        };

        let endpoint = if let Some(value) = self.http2_keep_alive_interval {
            endpoint.http2_keep_alive_interval(value)
        } else {
            endpoint
        };

        let endpoint = if let Some(value) = self.http2_keep_alive_timeout {
            endpoint.keep_alive_timeout(value)
        } else {
            endpoint
        };

        let endpoint = if let Some(value) = self.http2_keep_alive_while_idle {
            endpoint.keep_alive_while_idle(value)
        } else {
            endpoint
        };

        endpoint.tcp_keepalive(self.tcp_keepalive)
    }
}
