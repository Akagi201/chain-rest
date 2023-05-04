use near_jsonrpc_client::methods;
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::{BlockReference, Finality, FunctionArgs};
use near_primitives::views::QueryRequest;
use serde_json::{from_slice, json};

mod json_rpc;
use json_rpc::json_rpc_query_withtimeout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

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

    match json_rpc_query_withtimeout(&request, Some(100u64)).await {
        Ok(resp) => {
            // println!("{:#?}", resp);
            if let QueryResponseKind::CallResult(result) = resp.kind {
                println!("{:#?}", from_slice::<String>(&result.result)?);
            }
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }

    Ok(())
}
