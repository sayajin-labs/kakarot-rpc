use crate::providers::eth_provider::provider::{EthProviderResult, EthereumProvider1};
use async_trait::async_trait;
use auto_impl::auto_impl;
use reth_primitives::Address;
use reth_rpc_types::txpool::{TxpoolContent, TxpoolContentFrom, TxpoolInspect, TxpoolInspectSummary, TxpoolStatus};

#[async_trait]
#[auto_impl(Arc, &)]
pub trait PoolProvider {
    async fn txpool_status(&self) -> EthProviderResult<TxpoolStatus>;
    async fn txpool_inspect(&self) -> EthProviderResult<TxpoolInspect>;
    async fn txpool_content_from(&self, from: Address) -> EthProviderResult<TxpoolContentFrom>;
    async fn txpool_content(&self) -> EthProviderResult<TxpoolContent>;
}

#[derive(Debug, Clone)]
pub struct PoolDataProvider<P: EthereumProvider1> {
    eth_provider: P,
}

impl<P: EthereumProvider1> PoolDataProvider<P> {
    pub const fn new(eth_provider: P) -> Self {
        Self { eth_provider }
    }
}

#[async_trait]
impl<P: EthereumProvider1 + Send + Sync + 'static> PoolProvider for PoolDataProvider<P> {
    async fn txpool_status(&self) -> EthProviderResult<TxpoolStatus> {
        let all = self.eth_provider.txpool_content().await?;
        Ok(TxpoolStatus { pending: all.pending.len() as u64, queued: all.queued.len() as u64 })
    }

    async fn txpool_inspect(&self) -> EthProviderResult<TxpoolInspect> {
        let mut inspect = TxpoolInspect::default();

        let transactions = self.eth_provider.txpool_transactions().await?;

        for transaction in transactions {
            inspect.pending.entry(transaction.from).or_default().insert(
                transaction.nonce.to_string(),
                TxpoolInspectSummary {
                    to: transaction.to,
                    value: transaction.value,
                    gas: transaction.gas,
                    gas_price: transaction.gas_price.unwrap_or_default(),
                },
            );
        }

        Ok(inspect)
    }

    async fn txpool_content_from(&self, from: Address) -> EthProviderResult<TxpoolContentFrom> {
        Ok(self.eth_provider.txpool_content().await?.remove_from(&from))
    }

    async fn txpool_content(&self) -> EthProviderResult<TxpoolContent> {
        Ok(self.eth_provider.txpool_content().await?)
    }
}
