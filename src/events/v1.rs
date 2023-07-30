use crate::aggregates::v1::ProductAggregate;
use crate::commands::v1::CreateProductCommand;
use azure_core::Context;
use azure_data_cosmos::prelude::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use uuid::Uuid;

pub trait Event<TAggregate> {
    fn apply(&self, aggregate: TAggregate) -> TAggregate;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BaseEvent<TAggregate, TEvent> {
    pub id: String,
    pub kind: String,
    pub version: i32,
    pub aggregate: String,
    pub aggregate_id: String,
    pub created_by: String,
    #[serde(flatten)]
    pub event: TEvent,
    _marker: PhantomData<TAggregate>,
}

impl<TAggregate, TEvent> BaseEvent<TAggregate, TEvent> {
    pub fn new(
        id: String,
        kind: String,
        version: i32,
        aggregate: String,
        aggregate_id: String,
        created_by: String,
        event: TEvent,
    ) -> BaseEvent<TAggregate, TEvent> {
        BaseEvent {
            id,
            kind,
            version,
            aggregate,
            aggregate_id,
            created_by,
            event,
            _marker: PhantomData,
        }
    }
}

impl<TAggregate, TEvent> CosmosEntity for BaseEvent<TAggregate, TEvent> {
    type Entity = String;

    fn partition_key(&self) -> String {
        self.aggregate_id.to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductCreated {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: i32,
}

impl Event<ProductAggregate> for BaseEvent<ProductAggregate, ProductCreated> {
    fn apply(&self, mut aggregate: ProductAggregate) -> ProductAggregate {
        aggregate.id = Uuid::new_v4().to_string();
        aggregate.name = self.event.name.clone();
        aggregate.description = self.event.description.clone();
        aggregate.price = self.event.price.clone();
        aggregate.quantity = self.event.quantity.clone();
        aggregate.created_at = Utc::now();
        aggregate.created_by = self.created_by.clone();
        aggregate.modified_at = Utc::now();
        aggregate.modified_by = self.created_by.clone();
        aggregate
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductDeleted {
    pub deleted_at: u64,
}

impl Event<ProductAggregate> for BaseEvent<ProductAggregate, ProductDeleted> {
    fn apply(&self, mut aggregate: ProductAggregate) -> ProductAggregate {
        aggregate
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ProductEvent {
    ProductCreated(BaseEvent<ProductAggregate, ProductCreated>),
    ProductDeleted(BaseEvent<ProductAggregate, ProductDeleted>),
}
