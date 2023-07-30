use crate::aggregates::v1::ProductAggregate;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductCommand {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: i32,
    pub operator_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProductCommand {
    id: String,
    name: String,
    description: String,
    price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProductCommand {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SellProductCommand {
    id: String,
    quantity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RestockProductCommand {
    id: String,
    quantity: i32,
}
