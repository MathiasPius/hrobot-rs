use async_trait::async_trait;
use hyper::{
    client::{connect::Connect, HttpConnector},
    Body,
};
use hyper_rustls::HttpsConnector;
use serde::de::DeserializeOwned;
use tracing::trace;

use crate::{
    api::AuthenticatedRequest,
    error::{ApiResult, Error},
    AsyncRobot,
};

use super::r#async::AsyncClient;

impl Default for AsyncRobot<hyper::Client<HttpsConnector<HttpConnector>, Body>> {
    fn default() -> Self {
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();
        let client = hyper::Client::builder().build(https);

        Self::from_env(client).unwrap()
    }
}

impl<Response: 'static> TryInto<hyper::Request<Body>> for AuthenticatedRequest<Response> {
    type Error = hyper::http::Error;

    fn try_into(mut self) -> Result<hyper::Request<Body>, Self::Error> {
        let body = match self.take_body() {
            None => Body::empty(),
            Some(value) => Body::from(value),
        };

        hyper::Request::builder()
            .uri(self.uri())
            .method(self.method())
            .header("Authorization", self.authorization_header())
            .header("Content-Type", "application/x-www-form-urlencoded ")
            .header("Accept", "application/json")
            .body(body)
    }
}

#[async_trait]
impl<C> AsyncClient for hyper::Client<C, Body>
where
    C: Connect + Clone + Send + Sync + 'static,
{
    async fn send_request<Response>(
        &self,
        request: AuthenticatedRequest<Response>,
    ) -> Result<Response, Error>
    where
        Response: DeserializeOwned + Send + 'static,
    {
        let request = request.try_into().map_err(Error::transport)?;

        let response = self.request(request).await.map_err(Error::transport)?;

        let body = hyper::body::to_bytes(response.into_body())
            .await
            .map_err(Error::transport)?;

        let stringified = String::from_utf8_lossy(&body);
        trace!("response body: {stringified}");

        serde_json::from_str::<ApiResult<Response>>(&stringified)?.into()
    }
}
