use std::collections::HashMap;

use worker::*;

pub use console_error_panic_hook::set_once as set_panic_hook;

const DOH: &str = "https://security.cloudflare-dns.com/dns-query";
const DOH_JSON: &str = "https://security.cloudflare-dns.com/dns-query";
const CONTENT_TYPE: &str = "application/dns-message";
const ACCEPT_TYPE: &str = "application/dns-json";

#[event(fetch)]
async fn main(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    set_panic_hook();

    match req.method() {
        Method::Post if has_dns_content_type(req.headers()) => {
            let body = req.bytes().await?;
            reqwest::Client::new()
                .post(DOH)
                .header("Accept", CONTENT_TYPE)
                .header("Content-Type", CONTENT_TYPE)
                .body(body)
                .send()
                .await
                .map_err(map_reqwest_http_error)?;
        }

        Method::Get if has_dns_accept_type(req.headers()) => {
            if let Ok(url) = req.url() {
                let mut doh_json_url = Url::parse(DOH_JSON)?;
                doh_json_url.set_query(url.query());
                reqwest::Client::new()
                    .get(doh_json_url)
                    .header("Accept", ACCEPT_TYPE)
                    .send()
                    .await
                    .map_err(map_reqwest_http_error)?;
            }
        }

        Method::Get if has_dns_params(req.url()) => {
            if let Ok(url) = req.url() {
                let mut doh_url = Url::parse(DOH)?;
                doh_url.set_query(url.query());
                reqwest::Client::new()
                    .get(doh_url)
                    .header("Accept", CONTENT_TYPE)
                    .send()
                    .await
                    .map_err(map_reqwest_http_error)?;
            }
        }

        _ => {}
    }

    Response::error("Not Found", 404)
}

fn has_dns_content_type(headers: &Headers) -> bool {
    if let Ok(Some(content_type)) = headers.get("content-type") {
        return content_type == CONTENT_TYPE;
    }

    false
}

fn has_dns_accept_type(headers: &Headers) -> bool {
    if let Ok(Some(accept)) = headers.get("accept") {
        return accept == ACCEPT_TYPE;
    }

    false
}

fn has_dns_params(url_result: Result<Url>) -> bool {
    if let Ok(url) = url_result {
        let param: HashMap<_, _> = url.query_pairs().collect();
        return param.contains_key("dns");
    }

    false
}

fn map_reqwest_http_error(error: reqwest::Error) -> worker::Error {
    worker::Error::RustError(format!("reqwest::Error: {:?}", error))
}
