#![cfg(feature = "ureq")]

use ureq::{http::Response, Body};

use super::{HeaderMap, HttpResponse};
use crate::{Error, Result};

pub fn get(url: &str, headers: HeaderMap) -> Result<impl HttpResponse> {
    let mut req = ureq::get(url).header("user-agent", "rust-reqwest/self-update");

    for (key, value) in headers.into_iter() {
        if let Some(key) = key {
            req = req.header(key, value);
        }
    }

    let res = req.call()?;

    if !res.status().is_success() {
        bail!(
            Error::Network,
            "api request failed with status: {:?} - for: {:?}",
            res.status(),
            url
        )
    }

    res.headers();

    Ok(res)
}

impl HttpResponse for Response<Body> {
    fn headers(&self) -> &HeaderMap<http::HeaderValue> {
        Response::headers(&self)
    }

    fn status(&self) -> http::StatusCode {
        Response::status(&self)
    }

    fn body(self) -> impl std::io::Read {
        self.into_body().into_reader()
    }

    fn json<T: serde::de::DeserializeOwned>(mut self) -> Result<T> {
        Ok(self.body_mut().read_json::<T>()?)
    }

    fn text(mut self) -> Result<String> {
        Ok(self.body_mut().read_to_string()?)
    }
}
