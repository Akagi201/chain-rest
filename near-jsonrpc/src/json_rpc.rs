use near_jsonrpc_client::{methods, JsonRpcClient, MethodCallResult};
use near_jsonrpc_primitives::errors::RpcError;
use std::time::Duration;
use tokio::time::timeout;

const DEFAULT_JSON_RPC_CALL_TIMEOUT: u64 = 8;
pub const NEAR_TESTNET_RPC_URL: &str = "https://rpc.testnet.near.org";
pub const NEAR_TESTNET_ARCHIVAL_RPC_URL: &str = "https://archival-rpc.testnet.near.org";

pub async fn json_rpc_query_withtimeout<M>(
    method: &M,
    timeout_sec: Option<u64>,
) -> MethodCallResult<M::Response, M::Error>
where
    M: methods::RpcMethod,
    <M as methods::RpcMethod>::Error: std::fmt::Display,
{
    let secs = timeout_sec.unwrap_or(DEFAULT_JSON_RPC_CALL_TIMEOUT);

    let (rpc_url, archival_rpc_url) = (NEAR_TESTNET_RPC_URL, NEAR_TESTNET_ARCHIVAL_RPC_URL);

    match timeout(
        Duration::from_secs(secs),
        JsonRpcClient::connect(rpc_url).call(method),
    )
    .await
    {
        Ok(resp) => {
            return resp;
        }
        Err(err) => {
            println!(
                "json_rpc_call timeout for {} secs, err: {}, fallback to {}",
                secs, err, archival_rpc_url
            );
            match timeout(
                Duration::from_secs(secs),
                JsonRpcClient::connect(archival_rpc_url).call(method),
            )
            .await
            {
                Ok(resp) => {
                    return resp;
                }
                Err(err) => {
                    return Err(RpcError::new(
                        0,
                        format!("json_rpc_call timeout for {} secs, err: {}", secs, err),
                        None,
                    )
                    .into());
                }
            }
        }
    }
}
