use crate::structure::Request;

/// payload.toml の内容を検証する
pub fn validate_requests(requests: &[Request]) {
    println!("========================================");
    println!("        Payload Validation");
    println!("========================================");

    let mut warnings = 0;

    for request in requests {
        warnings += validate_request(request);
    }

    if warnings == 0 {
        println!("[OK] No problems found.");
    } else {
        println!();
        println!("[WARN] {} warning(s).", warnings);
    }

    println!("========================================");
    println!();
}

fn validate_request(request: &Request) -> usize {
    let mut warnings = 0;

    let method = request.method.to_uppercase();

    //
    // URL
    //
    if request.url.is_none() && request.base_url.is_none() {
        warn(request, "Neither url nor base_url specified.");
        warnings += 1;
    }

    //
    // GET/HEAD Body
    //
    if (method == "GET" || method == "HEAD") && request.body.is_some() {
        warn(request, "GET/HEAD request contains body.");
        warnings += 1;
    }

    //
    // POST/PUT/PATCH Body
    //
    if ["POST", "PUT", "PATCH"].contains(&method.as_str())
        && request.body.is_none()
        && request.multipart.is_none()
        && request.graphql.is_none()
    {
        warn(request, "POST/PUT/PATCH request has no body.");
        warnings += 1;
    }
    //
    // Content-Type
    //
    if request.body.is_some()
        && !request
            .headers
            .keys()
            .any(|k| k.eq_ignore_ascii_case("Content-Type"))
    {
        warn(request, "Body exists but Content-Type header is missing.");
        warnings += 1;
    }

    //
    // HTTP Version
    //
    if let Some(version) = &request.http_version {
        match version.as_str() {
            "auto" | "1" | "1.1" | "2" | "2.0" => {}

            _ => {
                warn(
                    request,
                    "http_version must be one of: auto, 1, 1.1, 2, 2.0.",
                );
                warnings += 1;
            }
        }
    }

    warnings
}

fn warn(request: &Request, message: &str) {
    println!(
        "[WARN] {} : {}",
        request.name.as_deref().unwrap_or("(unnamed)"),
        message
    );
}
