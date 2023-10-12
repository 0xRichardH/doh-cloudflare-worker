use std::{cmp, collections::HashMap};

use worker::*;

const DOH: &str = "https://security.cloudflare-dns.com/dns-query";
const DOH_JSON: &str = "https://security.cloudflare-dns.com/dns-query";
const CONTENT_TYPE: &str = "application/dns-message";
const ACCEPT_TYPE: &str = "application/dns-json";

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    match req.method() {
        Method::Post if has_dns_content_type(req.headers()) => {}
        Method::Get if has_dns_accept_type(req.headers()) => {}
        Method::Get if has_dns_params(req.url()) => {}
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
