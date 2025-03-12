use crate::api::request::QuestionReq;
use crate::api::response::{make_resp_json, Response};
use crate::operator::OperatorArc;
use actix_web::{body, get, post, web, Error, HttpRequest, HttpResponse, Result};
use alloy::primitives::{address, Address};
use alloy_wrapper::contracts::vrf_range;
use alloy_wrapper::util::recover_signer_alloy;
use node_api::error::ErrorCodes;
use node_api::error::{
    OperatorAPIError::APIFailToJson,
    OperatorError::{OPGetVrfRangeContractError, OPSendPromptError},
};
// use serde::{Deserialize, Serialize};
use serde_json::json;
use hex::FromHex;
use tracing::{debug, error, info};

/// WRITE API
// question input a prompt, and async return success, the answer callback later
#[post("/api/v1/question")]
async fn question(
    quest: web::Json<QuestionReq>,
    op: web::Data<OperatorArc>,
) -> web::Json<Response> {
    info!("Receive request, body = {:?}", quest);

    // todo: validate parameter and signature
    if !quest.signature.is_empty() && !quest.prompt_hash.is_empty() {
        let addr = recover_signer_alloy(quest.signature.clone(), &quest.prompt_hash);
        if let Err(err) = addr {
            error!("Validate signature error, detail = {:?}", err);
        } else {
            debug!("recovered addr : {:?}", addr.unwrap());
        }
    }

    // todo: move to others
    let bytes = <[u8; 20]>::from_hex(&op.config.node.node_id[2..]).unwrap_or_default();
    let addr: Address = Address::new(bytes);
    let threshold = vrf_range::get_range_by_address(op.vrf_range_contract.clone(), addr).await;
    if let Err(err) = threshold {
        return make_resp_json(
            quest.request_id.clone(),
            ErrorCodes::OP_GET_RANGE_CONTRACT_ERROR,
            OPGetVrfRangeContractError(err.to_string()).to_string(),
            serde_json::Value::default(),
        );
    }

    let json_data = json!({});
    make_resp_json(quest.request_id.clone(), 0, String::new(), json_data)
}
