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
            headers.append(header_name, header_value);
        }

        headers.insert(
            "Content-Type",
            "application/vnd.siren+json".parse().unwrap(),
        );

        response
    }
}
