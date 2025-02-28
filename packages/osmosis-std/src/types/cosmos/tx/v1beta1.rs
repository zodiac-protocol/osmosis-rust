use osmosis_std_derive::CosmwasmExt;
/// Tx is the standard type used for broadcasting transactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.Tx")]
pub struct Tx {
    /// body is the processable content of the transaction
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<TxBody>,
    /// auth_info is the authorization related content of the transaction,
    /// specifically signers, signer modes and fee
    #[prost(message, optional, tag = "2")]
    pub auth_info: ::core::option::Option<AuthInfo>,
    /// signatures is a list of signatures that matches the length and order of
    /// AuthInfo's signer_infos to allow connecting signature meta information like
    /// public key and signing mode by position.
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// TxRaw is a variant of Tx that pins the signer's exact binary representation
/// of body and auth_info. This is used for signing, broadcasting and
/// verification. The binary `serialize(tx: TxRaw)` is stored in Tendermint and
/// the hash `sha256(serialize(tx: TxRaw))` becomes the "txhash", commonly used
/// as the transaction ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.TxRaw")]
pub struct TxRaw {
    /// body_bytes is a protobuf serialization of a TxBody that matches the
    /// representation in SignDoc.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub body_bytes: ::prost::alloc::vec::Vec<u8>,
    /// auth_info_bytes is a protobuf serialization of an AuthInfo that matches the
    /// representation in SignDoc.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub auth_info_bytes: ::prost::alloc::vec::Vec<u8>,
    /// signatures is a list of signatures that matches the length and order of
    /// AuthInfo's signer_infos to allow connecting signature meta information like
    /// public key and signing mode by position.
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// SignDoc is the type used for generating sign bytes for SIGN_MODE_DIRECT.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.SignDoc")]
pub struct SignDoc {
    /// body_bytes is protobuf serialization of a TxBody that matches the
    /// representation in TxRaw.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub body_bytes: ::prost::alloc::vec::Vec<u8>,
    /// auth_info_bytes is a protobuf serialization of an AuthInfo that matches the
    /// representation in TxRaw.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub auth_info_bytes: ::prost::alloc::vec::Vec<u8>,
    /// chain_id is the unique identifier of the chain this transaction targets.
    /// It prevents signed transactions from being used on another chain by an
    /// attacker
    #[prost(string, tag = "3")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// account_number is the account number of the account in state
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub account_number: u64,
}
/// TxBody is the body of a transaction that all signers sign over.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.TxBody")]
pub struct TxBody {
    /// messages is a list of messages to be executed. The required signers of
    /// those messages define the number and order of elements in AuthInfo's
    /// signer_infos and Tx's signatures. Each required signer address is added to
    /// the list only the first time it occurs.
    /// By convention, the first required signer (usually from the first message)
    /// is referred to as the primary signer and pays the fee for the whole
    /// transaction.
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// memo is any arbitrary note/comment to be added to the transaction.
    /// WARNING: in clients, any publicly exposed text should not be called memo,
    /// but should be called `note` instead (see <https://github.com/cosmos/cosmos-sdk/issues/9122>).
    #[prost(string, tag = "2")]
    pub memo: ::prost::alloc::string::String,
    /// timeout is the block height after which this transaction will not
    /// be processed by the chain
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timeout_height: u64,
    /// extension_options are arbitrary options that can be added by chains
    /// when the default options are not sufficient. If any of these are present
    /// and can't be handled, the transaction will be rejected
    #[prost(message, repeated, tag = "1023")]
    pub extension_options: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// extension_options are arbitrary options that can be added by chains
    /// when the default options are not sufficient. If any of these are present
    /// and can't be handled, they will be ignored
    #[prost(message, repeated, tag = "2047")]
    pub non_critical_extension_options: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// AuthInfo describes the fee and signer modes that are used to sign a
/// transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.AuthInfo")]
pub struct AuthInfo {
    /// signer_infos defines the signing modes for the required signers. The number
    /// and order of elements must match the required signers from TxBody's
    /// messages. The first element is the primary signer and the one which pays
    /// the fee.
    #[prost(message, repeated, tag = "1")]
    pub signer_infos: ::prost::alloc::vec::Vec<SignerInfo>,
    /// Fee is the fee and gas limit for the transaction. The first signer is the
    /// primary signer and the one which pays the fee. The fee can be calculated
    /// based on the cost of evaluating the body and doing signature verification
    /// of the signers. This can be estimated via simulation.
    #[prost(message, optional, tag = "2")]
    pub fee: ::core::option::Option<Fee>,
}
/// SignerInfo describes the public key and signing mode of a single top-level
/// signer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.SignerInfo")]
pub struct SignerInfo {
    /// public_key is the public key of the signer. It is optional for accounts
    /// that already exist in state. If unset, the verifier can use the required \
    /// signer address for this position and lookup the public key.
    #[prost(message, optional, tag = "1")]
    pub public_key: ::core::option::Option<crate::shim::Any>,
    /// mode_info describes the signing mode of the signer and is a nested
    /// structure to support nested multisig pubkey's
    #[prost(message, optional, tag = "2")]
    pub mode_info: ::core::option::Option<ModeInfo>,
    /// sequence is the sequence of the account, which describes the
    /// number of committed transactions signed by a given address. It is used to
    /// prevent replay attacks.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
}
/// ModeInfo describes the signing mode of a single or nested multisig signer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.ModeInfo")]
pub struct ModeInfo {
    /// sum is the oneof that specifies whether this represents a single or nested
    /// multisig signer
    #[prost(oneof = "mode_info::Sum", tags = "1, 2")]
    pub sum: ::core::option::Option<mode_info::Sum>,
}
/// Nested message and enum types in `ModeInfo`.
pub mod mode_info {
    use osmosis_std_derive::CosmwasmExt;
    /// Single is the mode info for a single signer. It is structured as a message
    /// to allow for additional fields such as locale for SIGN_MODE_TEXTUAL in the
    /// future
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Message,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
        CosmwasmExt,
    )]
    #[proto_message(type_url = "/cosmos.tx.v1beta1.ModeInfo.Single")]
    pub struct Single {
        /// mode is the signing mode of the single signer
        #[prost(enumeration = "super::super::signing::v1beta1::SignMode", tag = "1")]
        #[serde(
            serialize_with = "crate::serde::as_str::serialize",
            deserialize_with = "crate::serde::as_str::deserialize"
        )]
        pub mode: i32,
    }
    /// Multi is the mode info for a multisig public key
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Message,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
        CosmwasmExt,
    )]
    #[proto_message(type_url = "/cosmos.tx.v1beta1.ModeInfo.Multi")]
    pub struct Multi {
        /// bitarray specifies which keys within the multisig are signing
        #[prost(message, optional, tag = "1")]
        pub bitarray:
            ::core::option::Option<super::super::super::crypto::multisig::v1beta1::CompactBitArray>,
        /// mode_infos is the corresponding modes of the signers of the multisig
        /// which could include nested multisig public keys
        #[prost(message, repeated, tag = "2")]
        pub mode_infos: ::prost::alloc::vec::Vec<super::ModeInfo>,
    }
    /// sum is the oneof that specifies whether this represents a single or nested
    /// multisig signer
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum Sum {
        /// single represents a single signer
        #[prost(message, tag = "1")]
        Single(Single),
        /// multi represents a nested multisig signer
        #[prost(message, tag = "2")]
        Multi(Multi),
    }
}
/// Fee includes the amount of coins paid in fees and the maximum
/// gas to be used by the transaction. The ratio yields an effective "gasprice",
/// which must be above some miminum to be accepted into the mempool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.Fee")]
pub struct Fee {
    /// amount is the amount of coins to be paid as a fee
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// gas_limit is the maximum gas that can be used in transaction processing
    /// before an out of gas error occurs
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_limit: u64,
    /// if unset, the first signer is responsible for paying the fees. If set, the specified account must pay the fees.
    /// the payer must be a tx signer (and thus have signed this field in AuthInfo).
    /// setting this field does *not* change the ordering of required signers for the transaction.
    #[prost(string, tag = "3")]
    pub payer: ::prost::alloc::string::String,
    /// if set, the fee payer (either the first signer or the value of the payer field) requests that a fee grant be used
    /// to pay fees instead of the fee payer's own balance. If an appropriate fee grant does not exist or the chain does
    /// not support fee grants, this will fail
    #[prost(string, tag = "4")]
    pub granter: ::prost::alloc::string::String,
}
/// GetTxsEventRequest is the request type for the Service.TxsByEvents
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.GetTxsEventRequest")]
pub struct GetTxsEventRequest {
    /// events is the list of transaction event type.
    #[prost(string, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    #[prost(enumeration = "OrderBy", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_by: i32,
}
/// GetTxsEventResponse is the response type for the Service.TxsByEvents
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.GetTxsEventResponse")]
pub struct GetTxsEventResponse {
    /// txs is the list of queried transactions.
    #[prost(message, repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<Tx>,
    /// tx_responses is the list of queried TxResponses.
    #[prost(message, repeated, tag = "2")]
    pub tx_responses: ::prost::alloc::vec::Vec<super::super::base::abci::v1beta1::TxResponse>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// BroadcastTxRequest is the request type for the Service.BroadcastTxRequest
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.BroadcastTxRequest")]
pub struct BroadcastTxRequest {
    /// tx_bytes is the raw transaction.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub tx_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "BroadcastMode", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub mode: i32,
}
/// BroadcastTxResponse is the response type for the
/// Service.BroadcastTx method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.BroadcastTxResponse")]
pub struct BroadcastTxResponse {
    /// tx_response is the queried TxResponses.
    #[prost(message, optional, tag = "1")]
    pub tx_response: ::core::option::Option<super::super::base::abci::v1beta1::TxResponse>,
}
/// SimulateRequest is the request type for the Service.Simulate
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.SimulateRequest")]
pub struct SimulateRequest {
    /// tx is the transaction to simulate.
    /// Deprecated. Send raw tx bytes instead.
    #[deprecated]
    #[prost(message, optional, tag = "1")]
    pub tx: ::core::option::Option<Tx>,
    /// tx_bytes is the raw transaction.
    ///
    /// Since: cosmos-sdk 0.43
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub tx_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// SimulateResponse is the response type for the
/// Service.SimulateRPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.SimulateResponse")]
pub struct SimulateResponse {
    /// gas_info is the information about gas used in the simulation.
    #[prost(message, optional, tag = "1")]
    pub gas_info: ::core::option::Option<super::super::base::abci::v1beta1::GasInfo>,
    /// result is the result of the simulation.
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<super::super::base::abci::v1beta1::Result>,
}
/// GetTxRequest is the request type for the Service.GetTx
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.GetTxRequest")]
pub struct GetTxRequest {
    /// hash is the tx hash to query, encoded as a hex string.
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
}
/// GetTxResponse is the response type for the Service.GetTx method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.v1beta1.GetTxResponse")]
pub struct GetTxResponse {
    /// tx is the queried transaction.
    #[prost(message, optional, tag = "1")]
    pub tx: ::core::option::Option<Tx>,
    /// tx_response is the queried TxResponses.
    #[prost(message, optional, tag = "2")]
    pub tx_response: ::core::option::Option<super::super::base::abci::v1beta1::TxResponse>,
}
/// OrderBy defines the sorting order
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum OrderBy {
    /// ORDER_BY_UNSPECIFIED specifies an unknown sorting order. OrderBy defaults to ASC in this case.
    Unspecified = 0,
    /// ORDER_BY_ASC defines ascending order
    Asc = 1,
    /// ORDER_BY_DESC defines descending order
    Desc = 2,
}
impl OrderBy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderBy::Unspecified => "ORDER_BY_UNSPECIFIED",
            OrderBy::Asc => "ORDER_BY_ASC",
            OrderBy::Desc => "ORDER_BY_DESC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_BY_UNSPECIFIED" => Some(Self::Unspecified),
            "ORDER_BY_ASC" => Some(Self::Asc),
            "ORDER_BY_DESC" => Some(Self::Desc),
            _ => None,
        }
    }
}
/// BroadcastMode specifies the broadcast mode for the TxService.Broadcast RPC method.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum BroadcastMode {
    /// zero-value for mode ordering
    Unspecified = 0,
    /// BROADCAST_MODE_BLOCK defines a tx broadcasting mode where the client waits for
    /// the tx to be committed in a block.
    Block = 1,
    /// BROADCAST_MODE_SYNC defines a tx broadcasting mode where the client waits for
    /// a CheckTx execution response only.
    Sync = 2,
    /// BROADCAST_MODE_ASYNC defines a tx broadcasting mode where the client returns
    /// immediately.
    Async = 3,
}
impl BroadcastMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BroadcastMode::Unspecified => "BROADCAST_MODE_UNSPECIFIED",
            BroadcastMode::Block => "BROADCAST_MODE_BLOCK",
            BroadcastMode::Sync => "BROADCAST_MODE_SYNC",
            BroadcastMode::Async => "BROADCAST_MODE_ASYNC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BROADCAST_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "BROADCAST_MODE_BLOCK" => Some(Self::Block),
            "BROADCAST_MODE_SYNC" => Some(Self::Sync),
            "BROADCAST_MODE_ASYNC" => Some(Self::Async),
            _ => None,
        }
    }
}
