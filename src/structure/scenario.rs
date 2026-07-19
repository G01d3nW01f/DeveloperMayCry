use crate::structure::{
    ExecutionSummary, Request, RuntimeOptions, Session, build_request, create_client,
    execute_request, print_dry_run, print_error, print_report, validate,
};

use std::error::Error;

/// Execute one request.
pub async fn execute_request_scenario(
    request: &Request,
    runtime: &RuntimeOptions,
    session: &mut Session,
    summary: &mut ExecutionSummary,
) -> Result<(), Box<dyn Error>> {
    let client = create_client(request, runtime)?;
    let builder = build_request(&client, request, session)?;

    //
    // Dry Run
    //
    if runtime.dry_run {
        print_dry_run(builder)?;
        return Ok(());
    }

    //
    // if = "variable"
    //
    if let Some(condition) = &request.r#if {
        let execute = if let Some(name) = condition.strip_prefix('!') {
            session.get_variable(name).is_none()
        } else {
            session.get_variable(condition).is_some()
        };

        if !execute {
            summary.total += 1;
            summary.skipped += 1;

            println!(
                "[-] Skip {}",
                request.name.as_deref().unwrap_or("(unnamed)")
            );

            return Ok(());
        }
    }

    println!("[+] Sending {} {}", request.method, request.url());

    let response = match execute_request(builder, request, session).await {
        Ok(r) => r,

        Err(err) => {
            summary.total += 1;
            summary.failed += 1;

            print_error(request, err.as_ref());

            if runtime.continue_on_error {
                return Ok(());
            }

            return Err(err);
        }
    };

    let result = validate(request, &response);

    summary.total += 1;

    if result.success() {
        summary.success += 1;
    } else {
        summary.failed += 1;
    }

    print_report(request, &response, &result);

    Ok(())
}
