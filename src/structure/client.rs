use crate::structure::definition::{Request, RuntimeOptions};

use reqwest::{Client, Proxy};

use std::{error::Error, time::Duration};

pub fn create_client(
    request: &Request,
    runtime: &RuntimeOptions,
) -> Result<Client, Box<dyn Error>> {
    //
    // Runtime Optionが優先
    //
    let verify_tls = if runtime.insecure {
        false
    } else {
        request.verify_tls
    };

    let timeout = runtime.timeout.unwrap_or(request.timeout);

    let mut builder = Client::builder()
        .danger_accept_invalid_certs(!verify_tls)
        .timeout(Duration::from_secs(timeout))
        .redirect(if request.follow_redirect {
            reqwest::redirect::Policy::limited(10)
        } else {
            reqwest::redirect::Policy::none()
        });

    //
    // HTTP Version
    //
    match request.http_version.as_deref() {
        //
        // Auto (default)
        //
        None | Some("auto") => {
            // reqwestのデフォルト(ALPNネゴシエーション)
        }

        //
        // HTTP/1.1
        //
        Some("1") | Some("1.1") => {
            builder = builder.http1_only();
        }

        //
        // HTTP/2 Prior Knowledge
        //
        Some("2") | Some("2.0") => {
            builder = builder.http2_prior_knowledge();
        }

        //
        // validator.rsで検出済み
        //
        _ => {}
    }

    //
    // Proxy
    //
    if let Some(proxy) = &runtime.proxy {
        builder = builder.proxy(Proxy::all(proxy)?);
    }

    Ok(builder.build()?)
}
