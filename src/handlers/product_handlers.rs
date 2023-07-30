use crate::aggregates::v1::ProductAggregate;
use crate::commands::v1::{
    CreateProductCommand, DeleteProductCommand, RestockProductCommand, SellProductCommand,
    UpdateProductCommand,
};
use crate::events::v1::{Event, ProductCreated};
use crate::{events, Aggregates};
use serde::de::Unexpected::Str;
use serde_json::Value::String;
use std::fmt::Error;
use uuid::Uuid;

pub struct ProductHandlers {
    collection: azure_data_cosmos::clients::CollectionClient,
}

impl ProductHandlers {
    pub fn new(collection: azure_data_cosmos::clients::CollectionClient) -> Self {
        ProductHandlers { collection }
    }

    pub async fn create_product(&self, cmd: CreateProductCommand) -> Result<(), ()> {
        let aggregate_id = Uuid::new_v4().to_string();
        let product_created = events::v1::BaseEvent::new(
            Uuid::new_v4().to_string(),
            "ProductCreated".to_string(),
            1,
            Aggregates::PRODUCT.to_string(),
            aggregate_id,
            cmd.operator_id.to_string(),
            ProductCreated {
                name: cmd.name,
                description: cmd.description,
                price: cmd.price,
                quantity: cmd.quantity,
            },
        );

        // Create empty aggregate.
        let aggregate = ProductAggregate::new();

        // Apply create_product.
        product_created.apply(aggregate);

        // Save event.
        match self.collection.create_document(product_created).await {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn update_product(cmd: UpdateProductCommand) {
        // Fetch event by aggregateId.

        // Loop events and build the aggregate.

        // Validation.

        // Apply update_product.

        // Append returned event.
    }

    pub fn delete_product(cmd: DeleteProductCommand) {
        // Delete events by aggregateId.
    }

    pub fn sell_product(cmd: SellProductCommand) {}

    pub fn restock_product(cmd: RestockProductCommand) {}
}
