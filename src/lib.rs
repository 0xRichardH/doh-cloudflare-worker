use once_cell::sync::Lazy;
use regex::Regex;
use worker::{js_sys::Uint8Array, *};

pub use console_error_panic_hook::set_once as set_panic_hook;

const DOH: &str = "https://1.1.1.1/dns-query";
const CONTENT_TYPE: &str = "application/dns-message";
const ACCEPT_TYPE: &str = "application/dns-json";

static DNS_PARAMS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"dns=").unwrap());

#[event(fetch)]
async fn main(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    set_panic_hook();

    match req.method() {
        Method::Post if has_dns_content_type(req.headers()) => {
            let body = req.bytes().await?;

            let mut headers = Headers::new();
            headers.set("Accept", CONTENT_TYPE)?;
            headers.set("Content-Type", CONTENT_TYPE)?;

            make_request(DOH, Method::Post, Some(headers), Some(body)).await
        }

        Method::Get if has_dns_accept_type(req.headers()) => {
            if let Ok(url) = req.url() {
                let mut doh_json_url = Url::parse(DOH)?;
                doh_json_url.set_query(url.query());

                let mut headers = Headers::new();
                headers.set("Accept", ACCEPT_TYPE)?;

                make_request(doh_json_url.as_str(), Method::Get, Some(headers), None).await
            } else {
                response_404()
            }
        }

        Method::Get if has_dns_params(req.url()) => {
            if let Ok(url) = req.url() {
                let mut doh_url = Url::parse(DOH)?;
                doh_url.set_query(url.query());

                let mut headers = Headers::new();
                headers.set("Accept", CONTENT_TYPE)?;

                make_request(doh_url.as_str(), Method::Get, Some(headers), None).await
            } else {
                response_404()
            }
        }

        _ => response_404(),
    }
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
        let Some(query) = url.query() else {
            return false;
        };

        return DNS_PARAMS_REGEX.is_match(query);
    }

    false
}

async fn make_request(
    url: &str,
    method: Method,
    headers: Option<Headers>,
    body: Option<Vec<u8>>,
) -> Result<Response> {
    let mut req_init = RequestInit::new();
    let mut req_init = req_init.with_method(method);

    if let Some(headers) = headers {
        req_init = req_init.with_headers(headers);
    }

    if let Some(body) = body {
        let body_as_js_value = Uint8Array::from(body.as_slice());
        req_init = req_init.with_body(Some(body_as_js_value.into()));
    }

    let request = Request::new_with_init(url, req_init)?;
    Fetch::Request(request).send().await
}

fn response_404() -> Result<Response> {
    Response::error("Not Found", 404)
}
