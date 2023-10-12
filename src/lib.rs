use std::collections::HashMap;

use reqwest::header;
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
            let resp = reqwest::Client::new()
                .post(DOH)
                .header("Accept", CONTENT_TYPE)
                .header("Content-Type", CONTENT_TYPE)
                .body(body)
                .send()
                .await
                .map_err(map_reqwest_http_error)?;
            response_ok(resp).await
        }

        Method::Get if has_dns_accept_type(req.headers()) => {
            if let Ok(url) = req.url() {
                let mut doh_json_url = Url::parse(DOH_JSON)?;
                doh_json_url.set_query(url.query());
                let resp = reqwest::Client::new()
                    .get(doh_json_url)
                    .header("Accept", ACCEPT_TYPE)
                    .send()
                    .await
                    .map_err(map_reqwest_http_error)?;
                response_ok(resp).await
            } else {
                response_404()
            }
        }

        Method::Get if has_dns_params(req.url()) => {
            if let Ok(url) = req.url() {
                let mut doh_url = Url::parse(DOH)?;
                doh_url.set_query(url.query());
                let resp = reqwest::Client::new()
                    .get(doh_url)
                    .header("Accept", CONTENT_TYPE)
                    .send()
                    .await
                    .map_err(map_reqwest_http_error)?;
                response_ok(resp).await
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
        let param: HashMap<_, _> = url.query_pairs().collect();
        return param.contains_key("dns");
    }

    false
}

fn map_reqwest_http_error(error: reqwest::Error) -> worker::Error {
    worker::Error::RustError(format!("reqwest::Error: {:?}", error))
}

fn map_reqwest_header_to_str_error(error: header::ToStrError) -> worker::Error {
    worker::Error::RustError(format!("reqwest::header::ToStrError: {:?}", error))
}

fn response_404() -> Result<Response> {
    Response::error("Not Found", 404)
}

async fn response_ok(resp: reqwest::Response) -> Result<Response> {
    // original response status
    let resp_status = resp.status();

    // original response headers
    let resp_headers = resp.headers();
    let mut response_header = worker::Headers::new();
    for (k, v) in resp_headers {
        let header_name = k.as_str();
        let header_value = v.to_str().map_err(map_reqwest_header_to_str_error)?;
        response_header.set(header_name, header_value)?;
    }

    // original response body
    let resp_bytes = resp.bytes().await.map_err(map_reqwest_http_error)?;
    let response_body = worker::ResponseBody::Body(resp_bytes.to_vec());

    // create new worker response
    let mut response = worker::Response::from_body(response_body)?;
    response = response.with_status(resp_status.as_u16());
    response = response.with_headers(response_header);

    Ok(response)
}
