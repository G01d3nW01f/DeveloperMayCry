use super::{
    Request, Session,
    extractor::{extract_cookies, extract_headers, extract_json, extract_regex},
};
use reqwest::{RequestBuilder, StatusCode, header::HeaderMap};
use std::{
    error::Error,
    time::{Duration, Instant},
};

/// HTTP response
#[derive(Debug)]
pub struct ResponseData {
    /// HTTP Status
    pub status: StatusCode,

    /// Headers
    pub headers: HeaderMap,

    /// Response Body
    pub body: String,

    /// Elapsed Time
    pub elapsed: Duration,
}

// Request sending
pub async fn execute_request(
    builder: RequestBuilder,
    request: &Request,
    session: &mut Session,
) -> Result<ResponseData, Box<dyn Error>> {
    let start = Instant::now();

    let response = builder.send().await?;

    // Cookie store to Session
    session.update_from_response(&response);

    let elapsed = start.elapsed();

    let status = response.status();

    let headers = response.headers().clone();

    let body = response.text().await?;

    // ------------------------------
    // Extractors
    // ------------------------------
    extract_headers(&headers, request, session);
    extract_cookies(&headers, request, session);
    extract_regex(&body, request, session);
    extract_json(&body, request, session);

    Ok(ResponseData {
        status,
        headers,
        body,
        elapsed,
    })
}
