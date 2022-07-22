#[cfg(feature = "axum")]
mod axum;

use http::{header::HeaderName, HeaderValue, StatusCode};
use serde::Serialize;

use super::Document;

/// Wrapper around an HTTP Siren response.
#[allow(dead_code)] // These fields are used by the various features.
pub struct Response<T>
where
    T: Serialize,
{
    status_code: StatusCode,
    headers:     Vec<(HeaderName, HeaderValue)>,
    document:    Document<T>,
}

impl<T> Response<T>
where
    T: Serialize,
{
    /// Create a new HTTP Siren response for the given Siren document.
    ///
    /// # Parameters
    /// - `document` - The document to wrap.
    pub fn new<D>(document: D) -> Self
    where
        D: Into<Document<T>>,
    {
        Self {
            status_code: StatusCode::OK,
            headers:     vec![],
            document:    document.into(),
        }
    }

    /// Specify the status code for the response.
    ///
    /// # Parameters
    /// - `status_code` - The status code.
    #[must_use]
    pub fn with_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;

        self
    }

    /// Specify an HTTP Header for the response.
    ///
    /// # Parameters
    /// - `header` - The header.
    #[must_use]
    pub fn with_header<H>(mut self, header: H) -> Self
    where
        H: Into<(HeaderName, HeaderValue)>,
    {
        self.headers.push(header.into());

        self
    }
}

impl<T> From<Document<T>> for Response<T>
where
    T: Serialize,
{
    fn from(document: Document<T>) -> Self {
        Self::new(document)
    }
}
