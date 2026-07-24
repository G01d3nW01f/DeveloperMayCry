use crate::structure::{
    Session,
    definition::{Cookie, Request},
};
use reqwest::header::HeaderMap;
use reqwest::header::SET_COOKIE;
use serde_json::Value;

/// Extract response headers into Session.variables
pub fn extract_headers(headers: &HeaderMap, request: &Request, session: &mut Session) {
    for (variable, header_name) in &request.extract.headers {
        if let Some(value) = headers.get(header_name) {
            if let Ok(text) = value.to_str() {
                session.set_variable(variable.clone(), text.to_string());
            }
        }
    }
}

pub fn extract_cookies(
    headers: &reqwest::header::HeaderMap,
    request: &Request,
    session: &mut Session,
) {
    for value in headers.get_all(SET_COOKIE).iter() {
        let Ok(cookie) = value.to_str() else {
            continue;
        };

        let Some(first) = cookie.split(';').next() else {
            continue;
        };

        let Some((cookie_name, cookie_value)) = first.split_once('=') else {
            continue;
        };

        //
        // CookieJar
        //
        session.cookie_jar.add(Cookie {
            name: cookie_name.to_string(),
            value: cookie_value.to_string(),

            domain: None,
            path: Some("/".to_string()),

            secure: false,
            http_only: false,

            expires: None,
        }); //
        // extract.cookies
        //
        for (variable, target_cookie) in &request.extract.cookies {
            if target_cookie.eq_ignore_ascii_case(cookie_name.trim()) {
                session.set_variable(variable.clone(), cookie_value.trim().to_string());
            }
        }
    }
}

use regex::Regex;

/// Extract values from response body using regex.
pub fn extract_regex(body: &str, request: &Request, session: &mut Session) {
    for (variable, pattern) in &request.extract.regex {
        let Ok(re) = Regex::new(pattern) else {
            continue;
        };

        let Some(caps) = re.captures(body) else {
            continue;
        };

        let Some(value) = caps.get(1) else {
            continue;
        };

        session.set_variable(variable.clone(), value.as_str().to_string());
    }
}

/// Extract values from JSON response.
pub fn extract_json(body: &str, request: &Request, session: &mut Session) {
    //
    // Nothing to do
    //
    if request.extract.json.is_empty() {
        return;
    }

    //
    // Parse JSON
    //
    let Ok(json): Result<Value, _> = serde_json::from_str(body) else {
        return;
    };

    //
    // JSONPath
    //
    for (variable, path) in &request.extract.json {
        let Ok(values) = jsonpath_lib::select(&json, path) else {
            continue;
        };

        if values.is_empty() {
            continue;
        }

        //
        // Build025
        //
        if values.len() == 1 {
            let value = values[0];

            if let Some(text) = value.as_str() {
                session.set_variable(variable.clone(), text.to_string());
            } else if let Some(obj) = value.as_object() {
                if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                    // extract inspection elements
                    if name.starts_with("__") {
                        continue;
                    }
                    // end section

                    session.set_variable(variable.clone(), name.to_string());
                }
            } else {
                session.set_variable(variable.clone(), value.to_string());
            }
        } else {
            let mut array = Vec::new();

            for value in values {
                if let Some(text) = value.as_str() {
                    array.push(text.to_string());
                } else if let Some(obj) = value.as_object() {
                    if let (Some(name), Some(kind)) = (
                        obj.get("name").and_then(|v| v.as_str()),
                        obj.get("kind").and_then(|v| v.as_str()),
                    ) {
                        //
                        // OBJECTだけ保存
                        //
                        if matches!(
                            kind,
                            "OBJECT" | "INPUT_OBJECT" | "INTERFACE" | "UNION" | "ENUM"
                        ) {
                            array.push(name.to_string());
                        }
                    }
                } else {
                    array.push(value.to_string());
                }
            }
            session.set_array(variable.clone(), array);
        }
    }
    //println!("Variables after JSON extract: {:#?}", session.variables);
}

pub fn extract_graphql(json: &serde_json::Value, _request: &Request, session: &mut Session) {
    //DBG
    //println!("extract_graphql() called");

    let Some(types) = json
        .pointer("/data/__schema/types")
        .and_then(|v| v.as_array())
    else {
        return;
    };

    let mut objects = Vec::new();

    for ty in types {
        let kind = ty.get("kind").and_then(|v| v.as_str());

        let name = ty.get("name").and_then(|v| v.as_str());

        if kind == Some("OBJECT") {
            if let Some(name) = name {
                objects.push(name.to_string());
            }
        }
    }

    //DBG
    if !objects.is_empty() {
        //println!("GraphQL OBJECTS = {:#?}", objects);
        session.set_array("graphql.objects", objects);
    }
}

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

pub fn extract_jwt(body: &str) {
    //
    // JSONをパース
    //
    let Ok(json) = serde_json::from_str::<Value>(body) else {
        return;
    };

    walk_json(&json);
}

fn walk_json(value: &Value) {
    match value {
        Value::Object(map) => {
            for (_, v) in map {
                if let Some(s) = v.as_str() {
                    inspect_jwt(s);
                }

                walk_json(v);
            }
        }

        Value::Array(arr) => {
            for v in arr {
                walk_json(v);
            }
        }

        _ => {}
    }
}

fn inspect_jwt(token: &str) {
    let parts: Vec<&str> = token.split('.').collect();

    if parts.len() != 3 {
        return;
    }

    let Ok(header) = URL_SAFE_NO_PAD.decode(parts[0]) else {
        return;
    };

    let Ok(payload) = URL_SAFE_NO_PAD.decode(parts[1]) else {
        return;
    };

    println!();
    println!("========================================");
    println!("JWT Detected");
    println!("========================================");

    println!("Header");
    println!("----------------------------------------");

    if let Ok(text) = String::from_utf8(header) {
        if let Ok(json) = serde_json::from_str::<Value>(&text) {
            println!("{}", serde_json::to_string_pretty(&json).unwrap());
        }
    }

    println!();

    println!("Payload");
    println!("----------------------------------------");

    if let Ok(text) = String::from_utf8(payload) {
        if let Ok(json) = serde_json::from_str::<Value>(&text) {
            println!("{}", serde_json::to_string_pretty(&json).unwrap());
        }
    }

    println!("========================================");
}
