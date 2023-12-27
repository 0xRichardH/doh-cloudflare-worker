use worker::{
    event, js_sys::Uint8Array, Context, Env, Fetch, Headers, Method, Request, RequestInit,
    Response, Result, Url,
};

pub use console_error_panic_hook::set_once as set_panic_hook;

const DOH: &str = "https://1.1.1.1/dns-query";
const DNS_MESSAGE: &str = "application/dns-message";
const DNS_JSON: &str = "application/dns-json";

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    set_panic_hook();

    match req.method() {
        Method::Post if has_dns_content_type(req.headers()) => post_dns_wireformat(req).await,

        Method::Get if has_dns_accept_type(req.headers()) => get_dns_json(req).await,

        Method::Get if has_dns_params(req.url()) => get_dns_wireformat(req).await,

        _ => response_404(),
    }
}

fn has_dns_content_type(headers: &Headers) -> bool {
    if let Ok(Some(content_type)) = headers.get("content-type") {
        return content_type == DNS_MESSAGE;
    }

    false
}

fn has_dns_accept_type(headers: &Headers) -> bool {
    if let Ok(Some(accept)) = headers.get("accept") {
        return accept == DNS_JSON;
    }

    false
}

fn has_dns_params(url_result: Result<Url>) -> bool {
    if let Ok(url) = url_result {
        let Some(query) = url.query() else {
            return false;
        };

        return query.contains("dns=");
    }

    false
}

async fn post_dns_wireformat(mut req: Request) -> Result<Response> {
    let body = req.bytes().await?;

    let mut headers = Headers::new();
    headers.set("Accept", DNS_MESSAGE)?;
    headers.set("Content-Type", DNS_MESSAGE)?;

    make_request(DOH, Method::Post, Some(headers), Some(&body)).await
}

async fn get_dns_wireformat(req: Request) -> Result<Response> {
    let Ok(url) = req.url() else {
        return response_404();
    };

    let mut doh_url = Url::parse(DOH)?;
    doh_url.set_query(url.query());

    let mut headers = Headers::new();
    headers.set("Accept", DNS_MESSAGE)?;

    make_request(doh_url.as_str(), Method::Get, Some(headers), None).await
}

async fn get_dns_json(req: Request) -> Result<Response> {
    let Ok(url) = req.url() else {
        return response_404();
    };

    let mut doh_json_url = Url::parse(DOH)?;
    doh_json_url.set_query(url.query());

    let mut headers = Headers::new();
    headers.set("Accept", DNS_JSON)?;

    make_request(doh_json_url.as_str(), Method::Get, Some(headers), None).await
}

async fn make_request(
    url: &str,
    method: Method,
    headers: Option<Headers>,
    body: Option<&[u8]>,
) -> Result<Response> {
    let mut req_init = RequestInit::new();
    let mut req_init = req_init.with_method(method);

    if let Some(headers) = headers {
        req_init = req_init.with_headers(headers);
    }

    if let Some(body) = body {
        let body_as_js_value = Uint8Array::from(body);
        req_init = req_init.with_body(Some(body_as_js_value.into()));
    }

    let request = Request::new_with_init(url, req_init)?;
    Fetch::Request(request).send().await
}

fn response_404() -> Result<Response> {
    Response::error("Not Found", 404)
}
