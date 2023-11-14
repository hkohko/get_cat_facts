use hyper;
use hyper::http;
use random_http_1::{build_client, build_req, process_req, ReqError};
use tokio;

#[tokio::main]
async fn main() -> Result<(), ReqError> {
    let _ = get_cat_facts("https://catfact.ninja/fact", http::Method::GET).await?;
    Ok(())
}
async fn get_cat_facts(url: &str, method: http::Method) -> Result<(), ReqError> {
    let uri: hyper::Uri = url.parse().unwrap_or_default();
    let req = build_req(&uri, method).await?;
    let https_client = build_client().await;
    let r = https_client.request(req).await?;
    process_req(r).await;
    Ok(())
}
