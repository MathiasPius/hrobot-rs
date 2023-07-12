//! Request Builders and Response Models.

use std::{marker::PhantomData, str::FromStr};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hyper::Uri;

mod server;
mod wrapper;

use serde::Serialize;
pub(crate) use server::*;

/// Base64-encoded credentials used to authenticate against
/// the Hetzner Robot API.
///
/// Used when [authenticating](UnauthenticatedRequest::authenticate)
/// a request, before it is transformed into a client-dependent request
/// type and sent.
#[derive(Clone)]
pub struct Credentials {
    pub header_value: String,
}

impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let encoded = &self.header_value.strip_prefix("Basic ").unwrap();
        let decoded = BASE64.decode(encoded).unwrap();
        let stringified = String::from_utf8_lossy(&decoded);

        let username = stringified.split_once(':').unwrap().0;

        f.debug_struct("Credentials")
            .field("username", &username)
            .field("password", &"<hidden>")
            .finish()
    }
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
pub struct UnauthenticatedRequest<Response> {
    /// URI for the resource.
    uri: Uri,
    /// HTTP Request Method. Should be one of GET, POST, PUT, or DELETE.
    method: &'static str,
    /// application/x-www-form-urlencoded body of the request.
    body: Option<String>,
    _response: PhantomData<Response>,
}

impl<Response> std::fmt::Debug for UnauthenticatedRequest<Response> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnauthenticatedRequest")
            .field("uri", &self.uri)
            .field("method", &self.method)
            .field("body", &self.body)
            .field(
                "response type",
                &std::any::type_name::<Response>().to_string(),
            )
            .finish()
    }
}

impl<Response> UnauthenticatedRequest<Response> {
    /// Construct a new [`UnauthenticatedRequest`] GET from a Uri.
    pub(crate) fn new(uri: Uri) -> Self {
        UnauthenticatedRequest {
            uri,
            method: "GET",
            body: None,
            _response: PhantomData,
        }
    }

    /// Construct an [`UnauthenticatedRequest`] from a plain-text URI.
    ///
    /// Panics if given an invalid URI string.
    pub(crate) fn from(uri: &str) -> Self {
        Self::new(Uri::from_str(uri).expect("constructing the uri should never fail."))
    }

    /// Set the HTTP Request Method of the request.
    pub(crate) fn with_method(mut self, method: &'static str) -> Self {
        self.method = method;
        self
    }

    /// Set the body of the request.
    ///
    /// Is automatically encoded as application/x-www-form-urlencoded.
    pub(crate) fn with_body<T: Serialize>(
        mut self,
        body: T,
    ) -> Result<Self, serde_html_form::ser::Error> {
        self.body = Some(serde_html_form::to_string(&body)?);
        Ok(self)
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
    request: UnauthenticatedRequest<Response>,
    credentials: Credentials,
}

impl<Response> AuthenticatedRequest<Response> {
    pub fn method(&self) -> &'static str {
        self.request.method
    }

    pub fn uri(&self) -> &Uri {
        &self.request.uri
    }

    pub fn authorization_header(&self) -> &str {
        &self.credentials.header_value
    }

    pub fn take_body(&mut self) -> Option<String> {
        self.request.body.take()
    }
}
