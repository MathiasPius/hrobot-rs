use std::{marker::PhantomData, str::FromStr};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hyper::Uri;

mod server;
mod wrapper;

use serde::Serialize;
pub use server::*;

/// Base64-encoded credentials used to authenticate against
/// the Hetzner Robot API.
///
/// Used when [authenticating](UnauthenticatedRequest::authenticate)
/// a request, before it is transformed into a client-dependent request
/// type and sent.
#[derive(Debug, Clone)]
pub struct Credentials {
    pub header_value: String,
}

impl Credentials {
    pub fn new(username: &str, password: &str) -> Self {
        let header = format!("Basic {}", BASE64.encode(format!("{username}:{password}")));

        Credentials {
            header_value: header,
        }
    }
}

/// Single API Request, and the expected `Response`.
///
/// Must be [`authenticated`](UnauthenticatedRequest::authenticate)
/// using Hetzner Robot [`Credentials`](Credentials) before it can be
/// transformed into a client-dependent request and then sent.
#[derive(Debug)]
pub struct UnauthenticatedRequest<Response> {
    pub uri: Uri,
    pub method: &'static str,
    pub body: Option<Vec<u8>>,
    pub headers: Vec<(&'static str, String)>,
    _response: PhantomData<Response>,
}

impl<Response> UnauthenticatedRequest<Response> {
    pub fn new(uri: Uri) -> Self {
        UnauthenticatedRequest {
            uri,
            method: "GET",
            body: None,
            headers: vec![],
            _response: PhantomData,
        }
    }

    pub fn from(uri: &str) -> Self {
        Self::new(Uri::from_str(uri).expect("constructing the uri should never fail."))
    }

    pub fn with_method(mut self, method: &'static str) -> Self {
        self.method = method;
        self
    }

    pub fn with_body<T: Serialize>(mut self, body: T) -> Result<Self, serde_json::Error> {
        self.body = Some(serde_json::to_vec(&body)?);
        Ok(self)
    }

    pub fn with_header(mut self, key: &'static str, value: &str) -> Self {
        self.headers.push((key, value.to_owned()));
        self
    }
}

impl<Response> UnauthenticatedRequest<Response> {
    pub fn authenticate(self, credentials: &Credentials) -> AuthenticatedRequest<Response> {
        AuthenticatedRequest {
            request: self,
            credentials: credentials.clone(),
        }
    }
}

/// API Request authenticated using Hetzner [`Credentials`](Credentials)
///
/// Use [`Into<YourClientRequestType>::into`] to transform the
/// [`AuthenticatedRequest`](AuthenticatedRequest) into a format usable
/// by your client library.
#[derive(Debug)]
pub struct AuthenticatedRequest<Response> {
    pub request: UnauthenticatedRequest<Response>,
    pub credentials: Credentials,
}
