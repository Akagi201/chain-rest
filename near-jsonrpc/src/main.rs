use near_jsonrpc_client::methods;
use near_jsonrpc_client::methods::query::RpcQueryRequest;
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::{BlockReference, Finality, FunctionArgs};
use near_primitives::views::QueryRequest;
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, json};
use std::collections::HashMap;

mod json_rpc;
use json_rpc::json_rpc_query_withtimeout;

#[derive(Debug, Serialize, Deserialize)]
struct GetUsersInfoParams {
    pub accounts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenBalance {
    pub balance: u64,
    pub pending_transfer: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerpPosition {
    pub position_qty: i64,
    pub cost_position: i64,
    pub last_sum_unitary_fundings: i64,
    pub last_executed_price: u64,
    pub last_settled_price: u64,
    pub average_entry_price: u64,
    pub opening_cost: i64,
    pub last_adl_price: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountInfo {
    pub token_balances: HashMap<String, TokenBalance>,
    pub perp_positions: HashMap<String, PerpPosition>,
    pub last_spot_trade_id: u64,
    pub last_perp_trade_id: u64,
    pub last_cefi_event_id: u64,
    pub last_deposit_event_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    if false {
        let request = methods::query::RpcQueryRequest {
            block_reference: BlockReference::Finality(Finality::Final),
            request: QueryRequest::CallFunction {
                account_id: "simple.statusmessage.raendev.testnet".parse()?,
                method_name: "get_status".to_string(),
                args: FunctionArgs::from(
                    json!({
                        "account_id": "akagi201.testnet".to_string(),
                    })
                    .to_string()
                    .into_bytes(),
                ),
            },
        };

        match json_rpc_query_withtimeout(&request, Some(10u64)).await {
            Ok(resp) => {
                println!("{:#?}", resp);
                if let QueryResponseKind::CallResult(result) = resp.kind {
                    println!("{:#?}", from_slice::<String>(&result.result)?);
                }
            }
            Err(err) => {
                println!("{:#?}", err);
            }
        }
    }

    let account_ids: Vec<String> = vec!["caorong.testnet".to_string()];
    let param = GetUsersInfoParams {
        accounts: account_ids.clone(),
    };
    let param_json = serde_json::to_string(&param)?;

    let request = RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::CallFunction {
            account_id: "asset-manager.orderly-perp-dev.testnet".parse()?,
            method_name: "get_users_info".to_string(),
            args: FunctionArgs::from(param_json.to_string().into_bytes()),
        },
    };

    match json_rpc_query_withtimeout(&request, Some(10u64)).await {
        Ok(resp) => {
            // println!("{:#?}", resp);
            if let QueryResponseKind::CallResult(result) = resp.kind {
                // println!("{:#?}", from_slice::<Vec<String>>(&result.result)?);
                // println!("{:?}", String::from_utf8(result.result).unwrap());
                let users_ledger = from_slice::<HashMap<String, AccountInfo>>(&result.result)?;
                println!("{:#?}", users_ledger);
            }
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }

    Ok(())
}
