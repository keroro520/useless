//! Wrappers for JSON serialization.
mod alert;
mod block_template;
mod blockchain;
mod bytes;
mod cell;
mod chain_info;
mod debug;
mod experiment;
mod fee_rate;
mod fixed_bytes;
mod net;
mod pool;
mod primitive;
mod proposal_short_id;
mod subscription;
mod uints;

#[cfg(test)]
mod tests;

pub use self::alert::{Alert, AlertId, AlertMessage, AlertPriority};
pub use self::block_template::{
    BlockTemplate, CellbaseTemplate, TransactionTemplate, UncleTemplate,
};
pub use self::blockchain::{
    Block, BlockEconomicState, BlockIssuance, BlockView, CellDep, CellInput, CellOutput, Consensus,
    DepType, EpochView, HardForkFeature, Header, HeaderView, MerkleProof, MinerReward, OutPoint,
    ProposalWindow, Script, ScriptHashType, Status, Transaction, TransactionProof, TransactionView,
    TransactionWithStatus, TxStatus, UncleBlock, UncleBlockView,
};
pub use self::bytes::JsonBytes;
pub use self::cell::{CellData, CellInfo, CellWithStatus};
pub use self::chain_info::ChainInfo;
pub use self::debug::{ExtraLoggerConfig, MainLoggerConfig};
pub use self::experiment::{DaoWithdrawingCalculationKind, DryRunResult};
pub use self::fee_rate::FeeRateDef;
pub use self::fixed_bytes::Byte32;
pub use self::net::{
    BannedAddr, LocalNode, LocalNodeProtocol, NodeAddress, PeerSyncState, RemoteNode,
    RemoteNodeProtocol, SyncState,
};
pub use self::pool::{
    OutputsValidator, PoolTransactionEntry, PoolTransactionReject, RawTxPool, TxPoolEntries,
    TxPoolEntry, TxPoolIds, TxPoolInfo,
};
pub use self::proposal_short_id::ProposalShortId;
pub use self::subscription::Topic;
pub use self::uints::{Uint128, Uint32, Uint64};
pub use primitive::{
    AsEpochNumberWithFraction, BlockNumber, Capacity, Cycle, EpochNumber, EpochNumberWithFraction,
    Timestamp, Version,
};
pub use serde::{Deserialize, Serialize};

/// This is a wrapper for JSON serialization to select the format between Json and Hex.
///
/// ## Examples
///
/// `ResponseFormat<BlockView, Block>` returns the block in its Json format or molecule serialized
/// Hex format.
pub enum ResponseFormat<V, P> {
    /// Serializes `V` as Json
    Json(V),
    /// Serializes `P` as Hex.
    ///
    /// `P` is first serialized by molecule into binary.
    ///
    /// The binary is then encoded as a 0x-prefixed hex string.
    Hex(P),
}

impl<V, P> Serialize for ResponseFormat<V, P>
where
    V: Serialize,
    P: ckb_types::prelude::Entity,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ResponseFormat::Json(view) => view.serialize(serializer),
            ResponseFormat::Hex(packed) => {
                let slice = packed.as_slice();
                let mut dst = vec![0u8; slice.len() * 2 + 2];
                dst[0] = b'0';
                dst[1] = b'x';
                faster_hex::hex_encode(slice, &mut dst[2..])
                    .map_err(|e| serde::ser::Error::custom(&format!("{}", e)))?;
                serializer.serialize_str(unsafe { ::std::str::from_utf8_unchecked(&dst) })
            }
        }
    }
}
