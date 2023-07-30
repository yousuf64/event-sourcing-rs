use crate::events::v1::{Event, ProductEvent};
use axum::Json;
use azure_data_cosmos::prelude::Query;
use chrono::format::Item::Error;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductAggregate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: i32,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    #[serde(with = "ts_milliseconds")]
    pub modified_at: DateTime<Utc>,
    pub modified_by: String,
}

impl ProductAggregate {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            price: 0.0,
            quantity: 0,
            created_at: Default::default(),
            created_by: "".to_string(),
            modified_at: Default::default(),
            modified_by: "".to_string(),
        }
    }

    pub async fn from_collection(
        collection: azure_data_cosmos::clients::CollectionClient,
        aggregate: String,
        aggregate_id: String,
    ) -> u64 {
        let query = Query::new(format!(
            "SELECT * FROM c WHERE c.aggregateId = '{}' AND c.aggregate = '{}'",
            aggregate_id, aggregate
        ));
        println!("Query {}", query.query());

        let mut aggregate_base = ProductAggregate::new();

        let mut stream = collection
            .query_documents(query)
            .into_stream::<ProductEvent>();

        while let Some(Ok(response)) = stream.next().await {
            for event in response.results {
                match event.0 {
                    ProductEvent::ProductCreated(event) => {
                        // println!("{:?}", event);
                        aggregate_base = event.apply(aggregate_base);
                    }
                    ProductEvent::ProductDeleted(event) => {
                        // println!("{:?}", event);
                        aggregate_base = event.apply(aggregate_base);
                    }
                }
            }
        }

        println!(
            "{}",
            serde_json::to_string_pretty(&json!(aggregate_base)).unwrap()
        );

        10
    }
}
