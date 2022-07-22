#[cfg(feature = "axum")]
mod axum;

use http::{HeaderMap, StatusCode};
use serde::Serialize;

use super::Document;

/// Wrapper around an HTTP Siren response.
pub struct Response<T>
where
    T: Serialize,
{
    pub status_code: StatusCode,
    pub headers:     HeaderMap,
    pub document:    Document<T>,
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
            headers:     HeaderMap::new(),
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

    /// Specify an HTTP Header to include in the response.
    ///
    /// # Parameters
    /// - `header` - The header to include.
    #[must_use]
    #[allow(clippy::needless_pass_by_value)] // Making this a reference just makes the caller slightly more awkward for no benefit
    pub fn with_header<H>(mut self, header: H) -> Self
    where
        H: headers_core::Header,
    {
        let mut values = vec![];
        header.encode(&mut values);

        for value in values {
            self.headers.append(H::name(), value);
        }

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
