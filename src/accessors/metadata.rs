use crate::{models::*, tables, txutil, Transaction};
use anyhow::Context;
use ethereum_types::H256;
use tracing::*;

pub async fn read_chain_config<'tx, Tx: Transaction<'tx>>(
    tx: &'tx Tx,
    block: H256,
) -> anyhow::Result<Option<ChainConfig>> {
    let key = block.as_bytes();

    trace!(
        "Reading chain config for block {:?} from at key {}",
        block,
        hex::encode(&key)
    );

    if let Some(b) = txutil::get_one::<_, tables::Config>(tx, &key).await? {
        trace!("Read chain config data: {}", hex::encode(&b));

        return Ok(Some(serde_json::from_slice(&*b).context("invalid JSON")?));
    }

    Ok(None)
}
