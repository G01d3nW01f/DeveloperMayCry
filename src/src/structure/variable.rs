use crate::structure::Session;
use regex::{Captures, Regex};

/// Expand variables.
///
/// Supported syntax:
///
/// {{name}}
/// {{name|default}}
/// {{ENV:NAME}}
/// {{ENV:NAME|default}}
pub fn expand_variables(input: &str, session: &Session) -> String {
    //println!("expand_variables(): input = {:?}", input);

    let re = Regex::new(r"\{\{([^}|]+)(?:\|([^}]+))?\}\}").expect("invalid variable regex");

    re.replace_all(input, |caps: &Captures| {
        let name = &caps[1];

        //println!("lookup variable = {:?}", name);
        //println!("session value    = {:?}", session.get_variable(name));
        //
        // Environment Variable
        //
        if let Some(env_name) = name.strip_prefix("ENV:") {
            if let Ok(value) = std::env::var(env_name) {
                return value;
            }
        }
        //
        // Session Variable
        //
        else if let Some(value) = session.get_variable(name) {
            //  println!("FOUND {} = {:?}", name, value);
            return value.clone();
        }

        //
        // Default Value
        //
        if let Some(default) = caps.get(2) {
            return default.as_str().to_string();
        }

        //
        // Not Found
        //
        //println!("NOT FOUND {}", name);

        String::new()
    })
    .into_owned()
}
