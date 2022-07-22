use axum::{response::IntoResponse, Json};
use serde::Serialize;

use super::Response;

impl<T> IntoResponse for Response<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let body = Json(self.document);
        let mut response = (self.status_code, body).into_response();

        let headers = response.headers_mut();

        for (header_name, header_value) in self.headers {
            if let Some(header_name) = header_name {
                headers.append(header_name, header_value);
            }
        }

        // We force the Content-Type to application/vnd.siren+json, even if a different one was
        // added.
        headers.insert(
            "Content-Type",
            "application/vnd.siren+json".parse().unwrap(),
        );

        response
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use assert2::check;
    use headers::{CacheControl, ContentType, ETag};
    use http::StatusCode;
    use insta::assert_json_snapshot;
    use serde_json::{json, Value};

    use super::*;
    use crate::{
        values::{FieldTypes, HttpMethods, LinkRelation},
        Action, Document, EmbeddedRepresentation, Field, Link,
    };

    #[tokio::test]
    async fn empty() {
        let document = crate::Document::new(json!({}));
        let response = crate::Response::new(document);

        let http_response = response.into_response();
        check!(http_response.status() == StatusCode::OK);
        check!(
            http_response.headers().get("Content-Type").unwrap() == "application/vnd.siren+json"
        );

        let body = hyper::body::to_bytes(http_response.into_body())
            .await
            .unwrap();
        check!(body.len() != 0);
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_json_snapshot!(body, @r###"
        {
          "properties": {}
        }
        "###)
    }

    #[tokio::test]
    async fn siren_example() {
        let document = Document::new(json!({
            "orderNumber": 42,
            "itemCount": 3,
            "status": "pending"
        }))
        .with_class("order")
        .with_embedded_link(
            Link::new("http://api.x.io/orders/42/items")
                .with_class("items")
                .with_class("collection")
                .with_rel("http://x.io/rels/order-items"),
        )
        .with_embedded_representation(
            EmbeddedRepresentation::new(json!({
                "customerId": "pj123",
                "name": "Peter Joseph"
            }))
            .with_class("info")
            .with_class("customer")
            .with_rel("http://x.io/rels/customer")
            .with_link(
                Link::new("http://api.x.io/customers/pj123").with_rel(LinkRelation::SelfLink),
            ),
        )
        .with_action(
            Action::new("add-item", "http://api.x.io/orders/42/items")
                .with_title("Add Item")
                .with_method(HttpMethods::POST)
                .with_type("application/x-www-form-urlencoded")
                .with_field(
                    Field::new("orderNumber")
                        .with_type(FieldTypes::Hidden)
                        .with_value("42"),
                )
                .with_field(Field::new("productCode").with_type(FieldTypes::Text))
                .with_field(Field::new("quantity").with_type(FieldTypes::Number)),
        )
        .with_link(Link::new("http://api.x.io/orders/42").with_rel(LinkRelation::SelfLink))
        .with_link(Link::new("http://api.x.io/orders/41").with_rel(LinkRelation::Previous))
        .with_link(Link::new("http://api.x.io/orders/43").with_rel(LinkRelation::Next));

        let response = crate::Response::new(document);

        let http_response = response.into_response();
        check!(http_response.status() == StatusCode::OK);
        check!(
            http_response.headers().get("Content-Type").unwrap() == "application/vnd.siren+json"
        );

        let body = hyper::body::to_bytes(http_response.into_body())
            .await
            .unwrap();
        check!(body.len() != 0);
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_json_snapshot!(body, @r###"
        {
          "class": [
            "order"
          ],
          "properties": {
            "orderNumber": 42,
            "itemCount": 3,
            "status": "pending"
          },
          "entities": [
            {
              "rel": [
                "http://x.io/rels/order-items"
              ],
              "class": [
                "items",
                "collection"
              ],
              "href": "http://api.x.io/orders/42/items"
            },
            {
              "rel": [
                "http://x.io/rels/customer"
              ],
              "class": [
                "info",
                "customer"
              ],
              "properties": {
                "customerId": "pj123",
                "name": "Peter Joseph"
              },
              "links": [
                {
                  "rel": [
                    "self"
                  ],
                  "href": "http://api.x.io/customers/pj123"
                }
              ]
            }
          ],
          "links": [
            {
              "rel": [
                "self"
              ],
              "href": "http://api.x.io/orders/42"
            },
            {
              "rel": [
                "previous"
              ],
              "href": "http://api.x.io/orders/41"
            },
            {
              "rel": [
                "next"
              ],
              "href": "http://api.x.io/orders/43"
            }
          ],
          "actions": [
            {
              "name": "add-item",
              "method": "POST",
              "href": "http://api.x.io/orders/42/items",
              "title": "Add Item",
              "type": "application/x-www-form-urlencoded",
              "fields": [
                {
                  "name": "orderNumber",
                  "type": "hidden",
                  "value": "42"
                },
                {
                  "name": "productCode",
                  "type": "text"
                },
                {
                  "name": "quantity",
                  "type": "number"
                }
              ]
            }
          ]
        }
        "###)
    }

    #[tokio::test]
    async fn status_code() {
        let document = crate::Document::new(json!({}));
        let response = crate::Response::new(document).with_status_code(StatusCode::ACCEPTED);

        let http_response = response.into_response();
        check!(http_response.status() == StatusCode::ACCEPTED);
        check!(
            http_response.headers().get("Content-Type").unwrap() == "application/vnd.siren+json"
        );

        let body = hyper::body::to_bytes(http_response.into_body())
            .await
            .unwrap();
        check!(body.len() != 0);
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_json_snapshot!(body, @r###"
        {
          "properties": {}
        }
        "###)
    }

    #[tokio::test]
    async fn headers() {
        let document = crate::Document::new(json!({}));
        let response = crate::Response::new(document)
            .with_status_code(StatusCode::OK)
            .with_header(
                CacheControl::new()
                    .with_public()
                    .with_max_age(std::time::Duration::from_secs(3600)),
            )
            .with_header(ETag::from_str("\"Hello\"").unwrap());

        let http_response = response.into_response();
        check!(http_response.status() == StatusCode::OK);
        check!(
            http_response.headers().get("Content-Type").unwrap() == "application/vnd.siren+json"
        );
        check!(http_response.headers().get("Cache-Control").unwrap() == "public, max-age=3600");
        check!(http_response.headers().get("ETag").unwrap() == "\"Hello\"");

        let body = hyper::body::to_bytes(http_response.into_body())
            .await
            .unwrap();
        check!(body.len() != 0);
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_json_snapshot!(body, @r###"
        {
          "properties": {}
        }
        "###)
    }

    #[tokio::test]
    async fn content_type_header() {
        let document = crate::Document::new(json!({}));
        let response = crate::Response::new(document)
            .with_status_code(StatusCode::OK)
            .with_header(ContentType::json());

        let http_response = response.into_response();
        check!(http_response.status() == StatusCode::OK);
        check!(
            http_response.headers().get("Content-Type").unwrap() == "application/vnd.siren+json"
        );

        let body = hyper::body::to_bytes(http_response.into_body())
            .await
            .unwrap();
        check!(body.len() != 0);
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_json_snapshot!(body, @r###"
        {
          "properties": {}
        }
        "###)
    }
}
