use crate::structure::{
    definition::{Request, Session},
    variable::expand_variables,
};

use reqwest::{
    Client, RequestBuilder,
    header::COOKIE,
    multipart::{Form, Part},
};

use std::error::Error;
pub fn build_request(
    client: &Client,
    request: &Request,
    session: &Session,
) -> Result<RequestBuilder, Box<dyn Error>> {
    let mut builder = client.request(request.http_method()?, request.url());

    builder = build_headers(builder, request, session);

    builder = build_cookies(builder, request, session);

    builder = build_query(builder, request, session);

    if request.multipart.is_some() {
        builder = build_multipart(builder, request, session)?;
    } else if request.graphql.is_some() {
        builder = build_graphql(builder, request, session)?;
    } else {
        builder = build_body(builder, request, session);
    }

    Ok(builder)
}

fn build_graphql(
    mut builder: RequestBuilder,
    request: &Request,
    session: &Session,
) -> Result<RequestBuilder, Box<dyn Error>> {
    let gql = request.graphql.as_ref().unwrap();

    //let variables = toml_to_json(&gql.variables, session);
    let variables = if let Some(v) = &gql.variables {
        toml_to_json(v, session)
    } else {
        serde_json::Value::Object(serde_json::Map::new())
    };
    let mut root = serde_json::Map::new();

    root.insert(
        "query".into(),
        serde_json::Value::String(expand_variables(&gql.query, session)),
    );

    root.insert("variables".into(), variables);

    if let Some(op) = &gql.operation_name {
        root.insert(
            "operationName".into(),
            serde_json::Value::String(op.clone()),
        );
    }

    builder = builder
        .header("Content-Type", "application/json")
        .json(&root);

    //println!("{}", serde_json::to_string_pretty(&root)?);
    Ok(builder)
}

fn build_multipart(
    builder: RequestBuilder,
    request: &Request,
    session: &Session,
) -> Result<RequestBuilder, Box<dyn Error>> {
    let multipart = request.multipart.as_ref().unwrap();

    let mut form = Form::new();

    //
    // Text Fields
    //
    for (name, value) in &multipart.fields {
        form = form.text(name.clone(), expand_variables(value, session));
    }

    //
    // Files
    //
    for (name, file) in &multipart.files {
        let bytes = std::fs::read(&file.path)?;

        let filename = file.filename.clone().unwrap_or_else(|| {
            std::path::Path::new(&file.path)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
        });

        let mut part = Part::bytes(bytes).file_name(filename);

        if let Some(content_type) = &file.content_type {
            part = part.mime_str(content_type)?;
        }

        form = form.part(name.clone(), part);
    }
    Ok(builder.multipart(form))
}

fn build_headers(
    mut builder: RequestBuilder,
    request: &Request,
    session: &Session,
) -> RequestBuilder {
    //println!("Session variables: {:#?}", session.variables);

    for (key, value) in &request.headers {
        let expanded = expand_variables(value, session);

        //println!("{} => {:?}", key, expanded);
        //println!("{:?}", key.as_bytes());

        builder = builder.header(key, expanded);
    }

    builder
}

fn build_cookies(
    mut builder: RequestBuilder,
    request: &Request,
    session: &Session,
) -> RequestBuilder {
    //
    // TOMLで指定されたCookie
    //
    let mut cookies = request.cookies.clone();

    //
    // URL情報
    //
    let url = request.url();

    let Ok(parsed) = reqwest::Url::parse(&url) else {
        return builder;
    };

    let is_https = parsed.scheme() == "https";

    let host = parsed.host_str().unwrap_or("");
    let path = parsed.path();
    //
    // CookieJarのCookieを追加
    // （TOML側が優先）
    //
    for cookie in &session.cookie_jar.cookies {
        //
        // Domain
        //
        if let Some(domain) = &cookie.domain {
            let domain = domain.trim_start_matches('.');

            if host != domain && !host.ends_with(&format!(".{}", domain)) {
                continue;
            }
        }
        //
        // Secure Cookie
        //
        if cookie.secure && !is_https {
            continue;
        }

        //
        // Path
        //
        if let Some(cookie_path) = &cookie.path {
            if !path.starts_with(cookie_path) {
                continue;
            }
        }

        if let Some(expire) = &cookie.expires {
            if expire.starts_with("Max-Age=0") {
                continue;
            }
        }

        cookies
            .entry(cookie.name.clone())
            .or_insert(cookie.value.clone());
    }

    //
    // Cookie Header生成
    //
    if !cookies.is_empty() {
        let header = cookies
            .iter()
            .map(|(k, v)| format!("{}={}", k, expand_variables(v, session)))
            .collect::<Vec<_>>()
            .join("; ");

        builder = builder.header(COOKIE, header);
    }

    builder
}

fn build_query(
    mut builder: RequestBuilder,
    request: &Request,
    session: &Session,
) -> RequestBuilder {
    if request.query.is_empty() {
        return builder;
    }

    let mut query = Vec::new();

    for (key, value) in &request.query {
        query.push((key.clone(), expand_variables(value, session)));
    }

    builder = builder.query(&query);

    builder
}

fn build_body(mut builder: RequestBuilder, request: &Request, session: &Session) -> RequestBuilder {
    //println!("build_body: {} {}", request.method, request.url());

    if let Some(body) = &request.body {
        let expanded = expand_variables(body, session);

        //println!("Expanded body = {:?}", expanded);

        builder = builder.body(expanded);
    } else {
        println!("No body");
    }

    builder
}

fn toml_to_json(value: &toml::Value, session: &Session) -> serde_json::Value {
    match value {
        toml::Value::String(s) => serde_json::Value::String(expand_variables(s, session)),

        toml::Value::Integer(i) => serde_json::Value::Number((*i).into()),

        toml::Value::Float(f) => {
            serde_json::json!(*f)
        }

        toml::Value::Boolean(b) => serde_json::Value::Bool(*b),

        toml::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(|v| toml_to_json(v, session)).collect())
        }

        toml::Value::Table(table) => {
            let mut obj = serde_json::Map::new();

            for (k, v) in table {
                obj.insert(k.clone(), toml_to_json(v, session));
            }

            serde_json::Value::Object(obj)
        }

        _ => serde_json::Value::Null,
    }
}
