use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::Height;
use tendermint_rpc::Url;

use crate::chain::cosmos::fetch_status_with_partial_parsing;
use crate::chain::endpoint::ChainStatus;
use crate::error::Error;

/// Query the chain status via an RPC query.
///
/// Returns an error if the node is still syncing and has not caught up,
/// ie. if `sync_info.catching_up` is `true`.
pub async fn query_status(chain_id: &ChainId, rpc_address: &Url) -> Result<ChainStatus, Error> {
    let response = fetch_status_with_partial_parsing(rpc_address).await?;

    if response.result.sync_info.catching_up {
        return Err(Error::chain_not_caught_up(
            rpc_address.to_string(),
            chain_id.clone(),
        ));
    }

    let time = response.result.sync_info.latest_block_time;

    let height = Height::new(
        ChainId::chain_version(response.result.node_info.network.as_str()),
        u64::from(response.result.sync_info.latest_block_height),
    )
    .map_err(|_| Error::invalid_height_no_source())?;

    Ok(ChainStatus {
        height,
        timestamp: time.into(),
    })
}
