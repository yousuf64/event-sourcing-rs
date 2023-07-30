mod aggregates;
mod commands;
mod events;
mod handlers;
mod responses;

use crate::events::v1::{BaseEvent, ProductCreated, ProductDeleted};
use crate::handlers::product_handlers;
use crate::handlers::product_handlers::ProductHandlers;
use crate::responses::v1::CreateProductResponse;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{http::StatusCode, Json, Router, ServiceExt};
use azure_core::prelude::App;
use azure_core::{Pageable, TransportOptions};
use azure_data_cosmos::clients::{
    CloudLocation, CollectionClient, CosmosClient, CosmosClientBuilder,
};
use azure_data_cosmos::prelude::{AuthorizationToken, Query, QueryDocumentsResponse};
use azure_data_cosmos::resources::permission::PermissionToken;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sonyflake::Sonyflake;
use std::arch::asm;
use std::string::ToString;
use std::sync::Arc;
use uuid::Uuid;

struct AppState {
    collection: CollectionClient,
    product_handlers: ProductHandlers,
}

#[tokio::main]
async fn main() {
    // let sf = Sonyflake::new().unwrap();
    // sf.next_id().unwrap();

    let primary_key =
        "C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==";
    let account = "https://localhost:8082/";
    let database_name = "event-sourcing";
    let collection_name = "events";

    // let authorization_token = AuthorizationToken::from(PermissionToken::try_from(primary_key)?)?;
    // let authorization_token = AuthorizationToken::primary_from_base64(&primary_key).unwrap();
    let http_client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let client = CosmosClientBuilder::with_location(CloudLocation::Emulator {
        address: "localhost".to_string(),
        port: 8082,
    })
    .transport(TransportOptions::new(Arc::new(http_client)))
    .build();
    let database = client.database_client(database_name);
    let collection = database.collection_client(collection_name);

    println!("getting collection");
    aggregates::v1::ProductAggregate::from_collection(
        collection,
        Aggregates::PRODUCT.to_string(),
        "7c24f9ce-fa98-448b-a5d8-8b052d12519d".to_string(),
    )
    .await;
    println!("got collection");

    // let query = collection
    //     .query_documents(Query::new("SELECT * FROM c".to_string()))
    //     .query_cross_partition(true)
    //     .max_item_count(1);

    // let mut stream = query.clone().into_stream::<BaseEvent<ProductDeleted>>();
    // let x = stream.next().await.unwrap();
    // println!("Structs: {:#?}", x.unwrap().results);

    // while let Some(respo) = stream.next().await {
    //     let respo = respo?;
    //     println!("Structs: {:#?}", respo.results);
    // }

    // let z = serde_json::from_value(stream).unwrap();
    // let resp = stream.next().await.unwrap();
    // let out = serde_json::from_reader(collections.collection).unwrap();
    // println!("{}", out);

    let app_state = Arc::new(AppState {
        collection: database.collection_client(collection_name),
        product_handlers: ProductHandlers::new(database.collection_client(collection_name)),
    });

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/products", post(create_product))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[non_exhaustive]
pub struct Aggregates;

impl Aggregates {
    pub const PRODUCT: &'static str = "PRODUCT";
}

#[axum_macros::debug_handler]
async fn create_product(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<commands::v1::CreateProductCommand>,
) -> (StatusCode, Json<CreateProductResponse>) {
    state
        .product_handlers
        .create_product(payload)
        .await
        .unwrap();

    (
        StatusCode::OK,
        Json::from(CreateProductResponse {
            aggregate_id: "aaa".to_string(),
        }),
    )
}
