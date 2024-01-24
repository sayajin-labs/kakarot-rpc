use reth_primitives::H256;
use reth_rpc_types::Transaction;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DbTransactionFull {
    #[serde(deserialize_with = "crate::db_client::types::serde::deserialize_intermediate")]
    pub tx: Transaction,
}

#[derive(Debug, Deserialize)]
pub struct DbTransactionHash {
    pub tx: Hash,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hash {
    pub block_hash: H256,
}

impl From<Hash> for H256 {
    fn from(hash: Hash) -> Self {
        hash.block_hash
    }
}
