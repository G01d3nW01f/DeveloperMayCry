use reqwest::Method;

use serde::{Deserialize, Serialize};

use std::{collections::HashMap, error::Error, str::FromStr};

/// payload.toml
#[derive(Debug, Deserialize)]
pub struct Config {
    pub requests: Vec<Request>,
}

/// HTTP Request
#[derive(Debug, Deserialize, Clone)]
pub struct Request {
    /// Request Name
    pub name: Option<String>,

    /// HTTP Method
    pub method: String,

    /// URL
    pub url: Option<String>,

    /// Base URL
    pub base_url: Option<String>,

    /// Path
    pub path: Option<String>,

    /// Header
    #[serde(default)]
    pub headers: HashMap<String, String>,

    /// Query
    #[serde(default)]
    pub query: HashMap<String, String>,

    /// Cookie
    #[serde(default)]
    pub cookies: HashMap<String, String>,

    /// Body
    pub body: Option<String>,

    /// TLS Verify
    #[serde(default = "default_verify_tls")]
    pub verify_tls: bool,

    /// Follow Redirect
    #[serde(default = "default_follow_redirect")]
    pub follow_redirect: bool,

    /// Timeout(sec)
    #[serde(default = "default_timeout")]
    pub timeout: u64,

    /// Validation
    pub expect: Option<Expect>,

    #[serde(default)]
    pub extract: Extract,

    #[serde(default)]
    pub r#if: Option<String>,

    #[serde(default)]
    pub http_version: Option<String>,

    #[serde(default)]
    pub foreach: Option<String>,

    #[serde(default)]
    pub multipart: Option<Multipart>,

    #[serde(default)]
    pub graphql: Option<GraphQL>,
}

/// Validation Rule
#[derive(Debug, Deserialize, Clone)]
pub struct Expect {
    pub status: Option<u16>,

    pub contains: Option<String>,

    pub not_contains: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct Extract {
    pub headers: HashMap<String, String>,

    pub cookies: HashMap<String, String>,
    /// Response Body (Regex)
    pub regex: HashMap<String, String>,

    /// JSONPath Extract
    pub json: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct Multipart {
    pub fields: HashMap<String, String>,
    pub files: HashMap<String, MultipartFile>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MultipartFile {
    pub path: String,

    #[serde(default)]
    pub filename: Option<String>,

    #[serde(default)]
    pub content_type: Option<String>,
}

/// Validation Result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub status: bool,

    pub contains: bool,

    pub not_contains: bool,
}

impl ValidationResult {
    pub fn success(&self) -> bool {
        self.status && self.contains && self.not_contains
    }
}

impl Request {
    /// Build URL
    pub fn url(&self) -> String {
        if let Some(url) = &self.url {
            url.clone()
        } else {
            format!(
                "{}{}",
                self.base_url.clone().unwrap_or_default(),
                self.path.clone().unwrap_or_default(),
            )
        }
    }

    /// HTTP Method
    pub fn http_method(&self) -> Result<Method, Box<dyn Error>> {
        Ok(Method::from_str(&self.method.to_uppercase())?)
    }
}

fn default_verify_tls() -> bool {
    false
}

fn default_follow_redirect() -> bool {
    true
}

fn default_timeout() -> u64 {
    30
}

//use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct RuntimeOptions {
    pub insecure: bool,

    pub timeout: Option<u64>,

    pub proxy: Option<String>,

    pub dry_run: bool,

    pub continue_on_error: bool,
}

//use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Session {
    pub cookie_jar: CookieJar,

    pub variables: HashMap<String, String>,

    pub arrays: HashMap<String, Vec<String>>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            cookie_jar: CookieJar::default(),
            variables: HashMap::new(),
            arrays: HashMap::new(),
        }
    }
}

impl Session {
    pub fn set_array(&mut self, name: impl Into<String>, values: Vec<String>) {
        self.arrays.insert(name.into(), values);
    }

    pub fn get_array(&self, name: &str) -> Option<&Vec<String>> {
        self.arrays.get(name)
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct ExecutionSummary {
    pub total: usize,
    pub success: usize,
    pub failed: usize,
    pub skipped: usize,
}

#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,

    pub domain: Option<String>,
    pub path: Option<String>,

    pub secure: bool,
    pub http_only: bool,
    pub expires: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CookieJar {
    pub cookies: Vec<Cookie>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct GraphQL {
    pub query: String,

    pub variables: Option<toml::Value>,

    pub operation_name: Option<String>,
}
