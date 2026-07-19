use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct OpenAPI {
    pub servers: Option<Vec<Server>>,
    pub paths: HashMap<String, PathItem>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct PathItem {
    pub get: Option<Operation>,
    pub post: Option<Operation>,
    pub put: Option<Operation>,
    pub delete: Option<Operation>,
    pub patch: Option<Operation>,

    #[serde(default)]
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Deserialize)]
pub struct Operation {
    pub summary: Option<String>,
}

use std::error::Error;

pub fn import_openapi(path: &str) -> Result<(), Box<dyn Error>> {
    let text = std::fs::read_to_string(path)?;

    let api: OpenAPI = serde_json::from_str(&text)?;
    //println!("{:#?}", api.paths);

    println!("========================================");
    println!("OpenAPI Import");
    println!("========================================");

    if let Some(servers) = &api.servers {
        println!("Servers");
        println!("----------------------------------------");

        for server in servers {
            println!("{}", server.url);
        }

        println!();
    }

    println!("Paths");
    println!("----------------------------------------");

    for (path, item) in &api.paths {
        if item.get.is_some() {
            println!("GET    {}", path);
        }

        if item.post.is_some() {
            println!("POST   {}", path);
        }

        if item.put.is_some() {
            println!("PUT    {}", path);
        }

        if item.patch.is_some() {
            println!("PATCH  {}", path);
        }

        if item.delete.is_some() {
            println!("DELETE {}", path);
        }
    }

    let requests = build_requests(&api);

    write_toml(&requests)?;

    println!();
    println!("Generated payload.toml");
    println!("Requests: {}", requests.len());

    Ok(())
}

fn convert_path(path: &str) -> String {
    let mut out = String::new();
    let mut chars = path.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' {
            out.push_str("{{");

            while let Some(ch) = chars.next() {
                if ch == '}' {
                    out.push_str("}}");
                    break;
                }

                out.push(ch);
            }
        } else {
            out.push(c);
        }
    }

    out
}

fn apply_parameters(req: &mut ImportedRequest, parameters: &[Parameter]) {
    for p in parameters {
        let value = format!("{{{{{}}}}}", p.name);

        match p.location.as_str() {
            "query" => {
                req.query.insert(p.name.clone(), value);
            }

            "header" => {
                req.headers.insert(p.name.clone(), value);
            }

            "cookie" => {
                req.cookies.insert(p.name.clone(), value);
            }

            _ => {}
        }
    }
}

fn build_requests(api: &OpenAPI) -> Vec<ImportedRequest> {
    let mut requests = Vec::new();

    let base = api
        .servers
        .as_ref()
        .and_then(|v| v.first())
        .map(|s| s.url.clone())
        .unwrap_or_default();

    for (path, item) in &api.paths {
        if let Some(op) = &item.get {
            requests.push(ImportedRequest {
                name: op
                    .summary
                    .clone()
                    .unwrap_or_else(|| format!("GET {}", path)),
                method: "GET".into(),
                url: format!("{}{}", base, convert_path(path)),
                query: HashMap::new(),
                headers: HashMap::new(),
                cookies: HashMap::new(),
            });

            let req = requests.last_mut().unwrap();
            apply_parameters(req, &item.parameters);
        }

        if let Some(op) = &item.post {
            requests.push(ImportedRequest {
                name: op
                    .summary
                    .clone()
                    .unwrap_or_else(|| format!("POST {}", path)),
                method: "POST".into(),
                url: format!("{}{}", base, convert_path(path)),
                query: HashMap::new(),
                headers: HashMap::new(),
                cookies: HashMap::new(),
            });
            let req = requests.last_mut().unwrap();
            apply_parameters(req, &item.parameters);
        }

        if let Some(op) = &item.put {
            requests.push(ImportedRequest {
                name: op
                    .summary
                    .clone()
                    .unwrap_or_else(|| format!("PUT {}", path)),
                method: "PUT".into(),
                url: format!("{}{}", base, convert_path(path)),
                query: HashMap::new(),
                headers: HashMap::new(),
                cookies: HashMap::new(),
            });
            let req = requests.last_mut().unwrap();
            apply_parameters(req, &item.parameters);
        }

        if let Some(op) = &item.patch {
            requests.push(ImportedRequest {
                name: op
                    .summary
                    .clone()
                    .unwrap_or_else(|| format!("PATCH {}", path)),
                method: "PATCH".into(),
                url: format!("{}{}", base, convert_path(path)),
                query: HashMap::new(),
                headers: HashMap::new(),
                cookies: HashMap::new(),
            });
            let req = requests.last_mut().unwrap();
            apply_parameters(req, &item.parameters);
        }

        if let Some(op) = &item.delete {
            requests.push(ImportedRequest {
                name: op
                    .summary
                    .clone()
                    .unwrap_or_else(|| format!("DELETE {}", path)),
                method: "DELETE".into(),
                url: format!("{}{}", base, convert_path(path)),
                query: HashMap::new(),
                headers: HashMap::new(),
                cookies: HashMap::new(),
            });
            let req = requests.last_mut().unwrap();
            apply_parameters(req, &item.parameters);
        }
    }

    requests
}

#[derive(Debug)]
pub struct ImportedRequest {
    pub name: String,
    pub method: String,
    pub url: String,

    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
}

fn write_toml(requests: &[ImportedRequest]) -> Result<(), Box<dyn Error>> {
    let mut out = String::new();

    for req in requests {
        out.push_str("[[requests]]\n");

        out.push_str(&format!("name={:?}\n", req.name));

        out.push_str(&format!("method={:?}\n", req.method));

        out.push_str(&format!("url={:?}\n", req.url));

        out.push_str(&format!("url={:?}\n", req.url));

        if !req.query.is_empty() {
            out.push_str("\n[requests.query]\n");

            for (k, v) in &req.query {
                out.push_str(&format!("{:?}={:?}\n", k, v));
            }
        }

        if !req.headers.is_empty() {
            out.push_str("\n[requests.headers]\n");

            for (k, v) in &req.headers {
                out.push_str(&format!("{:?}={:?}\n", k, v));
            }
        }

        if !req.cookies.is_empty() {
            out.push_str("\n[requests.cookies]\n");

            for (k, v) in &req.cookies {
                out.push_str(&format!("{:?}={:?}\n", k, v));
            }
        }
        out.push('\n');
    }

    std::fs::write("payload.toml", out)?;

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    pub name: String,

    #[serde(rename = "in")]
    pub location: String,

    pub required: Option<bool>,
}
