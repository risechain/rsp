use alloy_consensus::{Header, TxEnvelope};
use alloy_network::{Ethereum, Network};
use reth_ethereum_primitives::EthPrimitives;
use reth_optimism_primitives::OpPrimitives;
use reth_primitives_traits::NodePrimitives;

pub trait IntoPrimitives<N: Network>: NodePrimitives {
    fn into_primitive_block(block: N::BlockResponse) -> Self::Block;

    fn into_consensus_header(header: N::HeaderResponse) -> Header;
}

impl IntoPrimitives<Ethereum> for EthPrimitives {
    fn into_primitive_block(block: alloy_rpc_types::Block) -> Self::Block {
        let block = block.map_transactions(|tx| TxEnvelope::from(tx).into());
        block.into_consensus()
    }

    fn into_consensus_header(header: alloy_rpc_types::Header) -> Header {
        header.into()
    }
}

impl IntoPrimitives<op_alloy_network::Optimism> for OpPrimitives {
    fn into_primitive_block(
        block: alloy_rpc_types::Block<op_alloy_rpc_types::Transaction>,
    ) -> Self::Block {
        let block = block.map_transactions(|tx| tx.inner.inner.into_inner());
        block.into_consensus()
    }

    fn into_consensus_header(header: alloy_rpc_types::Header) -> Header {
        header.into()
    }
}
