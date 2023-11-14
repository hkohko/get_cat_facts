use hyper::body::HttpBody;
use hyper::client::HttpConnector;
use hyper::http;
use hyper_tls;
use hyper_tls::HttpsConnector;
use tokio;
use tokio::io::AsyncWriteExt;

pub async fn build_client() -> hyper::Client<HttpsConnector<HttpConnector>> {
    let https_client =
        hyper::Client::builder().build::<_, hyper::Body>(hyper_tls::HttpsConnector::new());
    https_client
}
pub async fn process_req(mut r: hyper::Response<hyper::Body>) {
    while let Some(chunk) = r.body_mut().data().await {
        tokio::io::stdout()
            .write_all(&chunk.unwrap_or_default())
            .await
            .unwrap_or_default();
    }
}
pub async fn build_req(
    uri: &hyper::Uri,
    method: http::Method,
) -> Result<hyper::Request<hyper::Body>, hyper::http::Error> {
    let req = hyper::Request::builder()
        .uri(uri)
        .method(method)
        .body(hyper::Body::empty());
    req
}
