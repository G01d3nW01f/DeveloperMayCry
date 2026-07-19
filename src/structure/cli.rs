use clap::Parser;

use std::path::PathBuf;

/// DeveloperMayCry CLI
#[derive(Debug, Parser)]
#[command(
    name = "DeveloperMayCry",
    version,
    author,
    about = "HTTP PoC Runner for Purple Team"
)]
pub struct Cli {
    /// Path to payload.toml
    #[arg(value_name = "PAYLOAD")]
    pub payload: PathBuf,

    /// Disable TLS certificate verification
    #[arg(short = 'k', long = "insecure")]
    pub insecure: bool,

    /// HTTP Proxy
    #[arg(long = "proxy")]
    pub proxy: Option<String>,

    /// Override Timeout
    #[arg(long = "timeout")]
    pub timeout: Option<u64>,

    /// Do not send HTTP request
    #[arg(long = "dry-run")]
    pub dry_run: bool,

    #[arg(long = "continue-on-error")]
    pub continue_on_error: bool,
}

use crate::structure::definition::RuntimeOptions;

impl Cli {
    pub fn runtime_options(&self) -> RuntimeOptions {
        RuntimeOptions {
            insecure: self.insecure,

            continue_on_error: self.continue_on_error,

            timeout: self.timeout,

            proxy: self.proxy.clone(),

            dry_run: self.dry_run,
        }
    }
}
