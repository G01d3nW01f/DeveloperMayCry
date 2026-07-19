use crate::structure::definition::{Cookie, CookieJar, Session};

use reqwest::header::SET_COOKIE;

impl CookieJar {
    pub fn add(&mut self, cookie: Cookie) {
        //
        // replace same Cookie
        //
        self.cookies.retain(|c| {
            !(c.name == cookie.name && c.domain == cookie.domain && c.path == cookie.path)
        });

        self.cookies.push(cookie);
    }
}

impl Session {
    /*pub fn new() -> Self {
        Self {
            cookie_jar: CookieJar::default(),
            variables: HashMap::new(),
            arrays: HashMap::new(),
        }
    }*/

    pub fn set_variable(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(name.into(), value.into());
    }

    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }
    /*
    pub fn set_array(&mut self, name: impl Into<String>, values: Vec<String>) {
        self.arrays.insert(name.into(), values);
    }*/
    /*
    pub fn get_array(&self, name: &str) -> Option<&Vec<String>> {
        self.arrays.get(name)
    }
    */
    pub fn update_from_response(&mut self, response: &reqwest::Response) {
        for value in response.headers().get_all(SET_COOKIE).iter() {
            let Ok(cookie) = value.to_str() else {
                continue;
            };

            let mut parts = cookie.split(';');

            let Some(first) = parts.next() else {
                continue;
            };

            let Some((name, value)) = first.split_once('=') else {
                continue;
            };

            let mut c = Cookie {
                name: name.trim().to_string(),
                value: value.trim().to_string(),

                domain: None,
                path: Some("/".to_string()),

                secure: false,
                http_only: false,

                expires: None,
            };

            for attr in parts {
                let attr = attr.trim();

                if attr.eq_ignore_ascii_case("Secure") {
                    c.secure = true;
                    continue;
                }

                if attr.eq_ignore_ascii_case("HttpOnly") {
                    c.http_only = true;
                    continue;
                }

                if let Some(v) = attr.strip_prefix("Domain=") {
                    c.domain = Some(v.to_string());
                    continue;
                }

                if let Some(v) = attr.strip_prefix("Path=") {
                    c.path = Some(v.to_string());
                    continue;
                }

                if let Some(v) = attr.strip_prefix("Expires=") {
                    c.expires = Some(v.to_string());
                    continue;
                }

                if let Some(v) = attr.strip_prefix("Max-Age=") {
                    c.expires = Some(format!("Max-Age={}", v));
                }
            }

            self.cookie_jar.add(c);
        }
    }
}
