//! Request Builders and Response Models.

use std::{marker::PhantomData, str::FromStr};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hyper::Uri;
use serde::Serialize;

mod wrapper;

pub mod boot;
pub mod failover;
pub mod firewall;
pub mod ip;
pub mod keys;
pub mod ordering;
pub mod rdns;
pub mod reset;
pub mod server;
pub mod storagebox;
pub mod subnet;
pub mod traffic;
pub mod vswitch;
pub mod wol;

/// Base64-encoded credentials used to authenticate against
/// the Hetzner Robot API.
///
/// Used by [`AsyncRobot`](crate::AsyncRobot) to authenticate requests, before it
/// is transformed into a client-dependent request type and sent.
#[derive(Clone)]
pub struct Credentials {
    header_value: String,
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
    /// Construct a new set of credentials from a username and password.
    ///
    /// # Example
    /// ```rust
    /// # use hrobot::api::Credentials;
    /// # fn main() {
    /// let credentials = Credentials::new("#ws+user", "p4ssw0rd");
    /// # }
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
pub(crate) struct UnauthenticatedRequest<Response> {
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

    /// Append the provided query parameters to the request.
    ///
    /// Panics if the query parameters are malformed.
    pub(crate) fn with_query_params<T: Serialize>(
        mut self,
        params: T,
    ) -> Result<Self, serde_html_form::ser::Error> {
        let params = serde_html_form::to_string(params)?;

        let mut parts = self.uri.into_parts();

        let path = parts
            .path_and_query
            .as_ref()
            .map(|paq| paq.path())
            .unwrap_or_default();

        let query = parts
            .path_and_query
            .as_ref()
            .and_then(|paq| paq.query())
            .unwrap_or_default();

        parts.path_and_query = Some(if query.is_empty() {
            format!("{path}?{params}").parse().unwrap()
        } else {
            format!("{path}?{query}&{params}").parse().unwrap()
        });

        self.uri = Uri::from_parts(parts).unwrap();

        Ok(self)
    }

    /// Set the body of the request.
    ///
    /// Is automatically encoded as application/x-www-form-urlencoded.
    pub(crate) fn with_body<T: Serialize>(
        self,
        body: T,
    ) -> Result<Self, serde_html_form::ser::Error> {
        Ok(self.with_serialized_body(serde_html_form::to_string(&body)?))
    }

    /// Set the body of the request.
    ///
    /// This assumes that the input string has already been serialized
    /// and therefore won't be url-encoded.
    ///
    /// Primarily used when configuring firewall rules since formatting
    /// the rule list is not supported by any serde url encoding library
    /// that I know of.
    pub(crate) fn with_serialized_body(mut self, body: String) -> Self {
        self.body = Some(body);
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
pub(crate) struct AuthenticatedRequest<Response> {
    request: UnauthenticatedRequest<Response>,
    credentials: Credentials,
}

impl<Response> AuthenticatedRequest<Response> {
    /// Returns the method of the request.
    ///
    /// One of `GET`, `POST`, `PUT` or `DELETE`.
    pub fn method(&self) -> &'static str {
        self.request.method
    }

    /// Returns the complete URI for the request.
    pub fn uri(&self) -> &Uri {
        &self.request.uri
    }

    /// Returns the already encoded header value to be used as
    /// the Authorization header of the request.
    ///
    /// Example: `"Basic aGVsbG86d29ybGQK"`
    pub fn authorization_header(&self) -> &str {
        &self.credentials.header_value
    }

    /// Returns the encoded body of the request.
    pub fn body(&self) -> Option<&str> {
        self.request.body.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::UnauthenticatedRequest;

    #[test]
    fn extend_query_parameters() {
        #[derive(Serialize)]
        struct QueryParams {
            example: &'static str,
        }

        assert_eq!(
            UnauthenticatedRequest::<()>::new("https://google.com?test=lol".parse().unwrap())
                .with_query_params(QueryParams {
                    example: "Some example here",
                })
                .unwrap()
                .uri,
            "https://google.com?test=lol&example=Some+example+here"
        );
    }
}
