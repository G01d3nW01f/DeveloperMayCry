mod openapi;
mod structure;
use clap::Parser;
//use std::env::args;
use crate::openapi::import_openapi;
use structure::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        print_help();
        return Ok(());
    }

    if args[1] == "import" {
        import_openapi(&args[2])?;
        return Ok(());
    }

    let cli = Cli::parse();

    let runtime = cli.runtime_options();

    let config = load_config(cli.payload.to_str().unwrap())?;

    //
    // Payload Validation
    //
    validate_requests(&config.requests);

    //
    // Session / Summary
    //
    let mut session = Session::new();
    let mut summary = ExecutionSummary::default();

    //
    // Execute Requests
    //
    for request in &config.requests {
        //
        // foreach
        //
        if let Some(array_name) = &request.foreach {
            if let Some(items) = session.get_array(array_name) {
                let items = items.clone();

                for item in items {
                    session.set_variable("item", item);

                    execute_request_scenario(request, &runtime, &mut session, &mut summary).await?;
                }
            } else {
                println!(
                    "[-] Skip {} (array '{}' not found)",
                    request.name.as_deref().unwrap_or("(unnamed)"),
                    array_name,
                );

                summary.total += 1;
                summary.skipped += 1;
            }

            continue;
        }

        //
        // Normal Request
        //
        execute_request_scenario(request, &runtime, &mut session, &mut summary).await?;
    }

    //
    // Execution Summary
    //
    print_summary(&summary);

    Ok(())
}

fn print_help() {
    println!("DeveloperMayCry");
    println!();
    println!("Usage:");
    println!("    dmc <payload.toml>");
    println!("    dmc import <openapi.json>");
    println!();
    println!("Examples:");
    println!("    dmc payloads/demo.toml");
    println!("    dmc import petstore.json");
}
