/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `delete_order`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOrderError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_inventory`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInventoryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_order_by_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrderByIdError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `place_order`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlaceOrderError {
    Status400(),
    UnknownValue(serde_json::Value),
}


pub fn delete_order(configuration: &configuration::Configuration, order_id: &str) -> Result<(), Error<DeleteOrderError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/store/order/{orderId}", configuration.base_path, orderId=crate::apis::urlencode(order_id));
    let mut req_builder = client.delete(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let mut resp = client.execute(req)?;

    let status = resp.status();
    let content = resp.text()?;

    if status.is_success() {
        Ok(())
    } else {
        let entity: Option<DeleteOrderError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

pub fn get_inventory(configuration: &configuration::Configuration, ) -> Result<::std::collections::HashMap<String, i32>, Error<GetInventoryError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/store/inventory", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let val = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("api_key", val);
    };

    let req = req_builder.build()?;
    let mut resp = client.execute(req)?;

    let status = resp.status();
    let content = resp.text()?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetInventoryError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

pub fn get_order_by_id(configuration: &configuration::Configuration, order_id: i64) -> Result<crate::models::Order, Error<GetOrderByIdError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/store/order/{orderId}", configuration.base_path, orderId=order_id);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let mut resp = client.execute(req)?;

    let status = resp.status();
    let content = resp.text()?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetOrderByIdError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

pub fn place_order(configuration: &configuration::Configuration, body: crate::models::Order) -> Result<crate::models::Order, Error<PlaceOrderError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/store/order", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&body);

    let req = req_builder.build()?;
    let mut resp = client.execute(req)?;

    let status = resp.status();
    let content = resp.text()?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<PlaceOrderError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

