use hyper;
use hyper::body::HttpBody;
use hyper_tls;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use tokio;
use tokio::io::AsyncWriteExt;
#[tokio::main]
async fn main() {
    let _ = sketch("https://catfact.ninja/fact", hyper::Method::GET).await;
}
async fn build_req(uri: &hyper::Uri, method: hyper::Method) -> Result<hyper::Request<hyper::Body>, hyper::http::Error>{
    let req = hyper::Request::builder()
    .uri(uri)
    .method(method)
    .body(hyper::Body::empty());
    req
}
async fn build_client() -> hyper::Client<HttpsConnector<HttpConnector>> {
    let https_client = hyper::Client::builder().build::<_, hyper::Body>(hyper_tls::HttpsConnector::new());
    https_client
}
async fn process_req(mut r: hyper::Response<hyper::Body>) {
    while let Some(chunk) = r.body_mut().data().await {
        tokio::io::stdout()
            .write_all(&chunk.unwrap_or_default())
            .await
            .unwrap_or_default();
    }
}
async fn sketch(url: &str, method: hyper::Method) {
    let uri: hyper::Uri = url.parse().unwrap_or_default();
    let req = build_req(&uri, method).await.unwrap_or_default();
    let https_client = build_client().await;
    let r = https_client.request(req).await.unwrap_or_default();
    process_req(r).await;
}