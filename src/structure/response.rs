use super::{
    Request, Session,
    extractor::{
        extract_cookies, extract_graphql, extract_headers, extract_json, extract_jwt, extract_regex,
    },
    variable::expand_variables,
};
use reqwest::{RequestBuilder, StatusCode, header::HeaderMap};
use std::{
    error::Error,
    fs,
    path::Path,
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

    //DBG
    let req = builder.try_clone().unwrap().build()?;

    println!("Headers:");
    for (k, v) in req.headers() {
        println!("{}: {:?}", k, v);
    }

    let response = builder.send().await?;

    // Cookie store to Session
    session.update_from_response(&response);

    let elapsed = start.elapsed();

    let status = response.status();

    let headers = response.headers().clone();

    let body = response.text().await?;

    let json = serde_json::from_str::<serde_json::Value>(&body).ok();

    // ------------------------------
    // Extractors
    // ------------------------------
    extract_headers(&headers, request, session);
    extract_cookies(&headers, request, session);
    extract_regex(&body, request, session);
    extract_json(&body, request, session);
    extract_jwt(&body);

    if let Some(ref json) = json {
        extract_graphql(json, request, session);
    }

    //
    // Save response
    //
    if let Some(output) = &request.output {
        let directory = expand_variables(&output.directory, session);
        let filename = expand_variables(&output.filename, session);

        fs::create_dir_all(&directory)?;

        let path = Path::new(&directory).join(filename);

        fs::write(&path, &body)?;

        println!("[+] Saved response -> {}", path.display());
    }

    Ok(ResponseData {
        status,
        headers,
        body,
        elapsed,
    })
}
