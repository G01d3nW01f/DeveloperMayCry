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
        // CookieJarへ保存
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
            } else {
                session.set_variable(variable.clone(), value.to_string());
            }
        } else {
            let mut array = Vec::new();

            for value in values {
                if let Some(text) = value.as_str() {
                    array.push(text.to_string());
                } else {
                    array.push(value.to_string());
                }
            }
            session.set_array(variable.clone(), array);
        }
    }
    println!("Variables after JSON extract: {:#?}", session.variables);
}
