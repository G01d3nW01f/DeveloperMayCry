use crate::structure::{
    definition::{Request, ValidationResult},
    response::ResponseData,
};

/// Console Report
pub fn print_report(request: &Request, response: &ResponseData, result: &ValidationResult) {
    println!();

    println!("====================================================");

    println!(
        "PoC          : {}",
        request.name.as_deref().unwrap_or("Unnamed")
    );

    println!("Method       : {}", request.method);

    println!(
        "HTTP         : {}",
        request.http_version.as_deref().unwrap_or("auto"),
    );

    println!("URL          : {}", request.url());

    println!("Status       : {}", response.status);

    println!("Elapsed      : {} ms", response.elapsed.as_millis());

    println!("Body Length  : {} bytes", response.body.len());

    println!("====================================================");

    println!();

    println!("Validation");

    println!("----------------------------------------");

    println!(
        "Status       {}",
        if result.status { "PASS" } else { "FAIL" }
    );

    println!(
        "Contains     {}",
        if result.contains { "PASS" } else { "FAIL" }
    );

    println!(
        "NotContains  {}",
        if result.not_contains { "PASS" } else { "FAIL" }
    );

    println!("----------------------------------------");

    println!();

    if result.success() {
        println!("========================================");
        println!("          REPRODUCED");
        println!("========================================");
    } else {
        println!("========================================");
        println!("        NOT REPRODUCED");
        println!("========================================");
    }

    println!();

    println!("Response Headers");

    println!("----------------------------------------");

    for (name, value) in &response.headers {
        println!("{}: {}", name, value.to_str().unwrap_or("<binary>"));
    }

    println!();

    println!("Response Body");

    println!("----------------------------------------");

    println!("{}", response.body);

    println!("----------------------------------------");
}

use reqwest::RequestBuilder;

pub fn print_dry_run(builder: RequestBuilder) -> Result<(), reqwest::Error> {
    let request = builder.build()?;

    println!();

    println!("========================================");
    println!("             DRY RUN");
    println!("========================================");

    println!("Method : {}", request.method());

    println!("URL    : {}", request.url());

    println!();

    println!("Headers");

    println!("----------------------------------------");

    for (name, value) in request.headers() {
        println!("{}: {}", name, value.to_str().unwrap_or("<binary>"),);
    }

    if let Some(body) = request.body() {
        println!();

        println!("Body");

        println!("----------------------------------------");

        if let Some(bytes) = body.as_bytes() {
            println!("{}", String::from_utf8_lossy(bytes));
        } else {
            println!("<stream body>");
        }
    }

    println!("----------------------------------------");

    Ok(())
}

pub fn print_error(request: &Request, error: &dyn std::error::Error) {
    println!();
    println!("========================================");
    println!(
        "PoC          : {}",
        request.name.as_deref().unwrap_or("(unnamed)")
    );
    println!("Method       : {}", request.method);
    println!("URL          : {}", request.url());

    println!("----------------------------------------");
    println!("FAILED");
    println!("{}", error);

    println!("========================================");
}

use crate::structure::ExecutionSummary;

pub fn print_summary(summary: &ExecutionSummary) {
    println!();

    println!("========================================");
    println!("Execution Summary");
    println!("========================================");

    println!("Total      : {}", summary.total);
    println!("Success    : {}", summary.success);
    println!("Failed     : {}", summary.failed);
    println!("Skipped    : {}", summary.skipped);

    println!("========================================");
}
