use hyper;
#[derive(Debug)]
pub enum ReqError {
    HTTPError(hyper::http::Error),
    HyperError(hyper::Error),
}
impl From<hyper::http::Error> for ReqError {
    fn from(err: hyper::http::Error) -> Self {
        Self::HTTPError(err)
    }
}
impl From<hyper::Error> for ReqError {
    fn from(err: hyper::Error) -> Self {
        Self::HyperError(err)
    }
}
