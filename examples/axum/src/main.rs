use axum::{routing::get, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(example));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn example() -> http_siren::Response<OrderProperties> {
    let document = http_siren::Document::new(OrderProperties {
        order_number: 42,
        item_count: 3,
        status: "pending".to_owned(),
    })
    .with_class("order")
    .with_embedded_link(
        http_siren::Link::new("http://api.x.io/orders/42/items")
            .with_class("items")
            .with_class("collection")
            .with_rel("http://x.io/rels/order-items"),
    )
    .with_embedded_representation(
        http_siren::EmbeddedRepresentation::new(EmbeddedCustomerProperties {
            customer_id: "pj123".to_owned(),
            name: "Peter Joseph".to_owned(),
        })
        .with_class("info")
        .with_class("customer")
        .with_rel("http://x.io/rels/customer")
        .with_link(
            http_siren::Link::new("http://api.x.io/customers/pj123")
                .with_rel(http_siren::values::LinkRelation::SelfLink),
        ),
    )
    .with_action(
        http_siren::Action::new("add-item", "http://api.x.io/orders/42/items")
            .with_title("Add Item")
            .with_method(http_siren::values::HttpMethods::POST)
            .with_type("application/x-www-form-urlencoded")
            .with_field(
                http_siren::Field::new("orderNumber")
                    .with_type(http_siren::values::FieldTypes::Hidden)
                    .with_value("42"),
            )
            .with_field(
                http_siren::Field::new("productCode")
                    .with_type(http_siren::values::FieldTypes::Text),
            )
            .with_field(
                http_siren::Field::new("quantity")
                    .with_type(http_siren::values::FieldTypes::Number),
            ),
    )
    .with_link(
        http_siren::Link::new("http://api.x.io/orders/42")
            .with_rel(http_siren::values::LinkRelation::SelfLink),
    )
    .with_link(
        http_siren::Link::new("http://api.x.io/orders/41")
            .with_rel(http_siren::values::LinkRelation::Previous),
    )
    .with_link(
        http_siren::Link::new("http://api.x.io/orders/43")
            .with_rel(http_siren::values::LinkRelation::Next),
    );

    http_siren::Response::new(document).with_header(
        headers::CacheControl::new()
            .with_public()
            .with_max_age(std::time::Duration::from_secs(3600)),
    )
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OrderProperties {
    pub order_number: u32,
    pub item_count: u32,
    pub status: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EmbeddedCustomerProperties {
    customer_id: String,
    name: String,
}
