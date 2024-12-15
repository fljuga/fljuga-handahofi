//! ## fljúga handahófi mlir codegen
//!
//! *fljúga handahófi* is a reference implementation of *rustc_codegen_mlir*,
//! a code generator targeting [LLVM MLIR](https://mlir.llvm.org/) Transformations and Dialects.
//!
//! *fljuga-handahofi-mlir-codegen* generates rust bindings for [mlir-c](https://mlir.llvm.org/docs/CAPI/) API using LLVM TableGen format.
//!
//! Http Client Module.
//!

use http_body_util::*;
use hyper::body::Bytes;
use hyper::header::ToStrError;
use hyper::http::uri::InvalidUri;
use hyper::{Method, Request, StatusCode, Uri};
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client as HyperClient;
use hyper_util::rt::TokioExecutor;

/// Derived [thiserror::Error] for hyper errors
#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] InvalidUri),

    #[error("{0}")]
    BackendError(#[from] hyper::Error),

    #[error("{0}")]
    HttpError(#[from] hyper::http::Error),

    #[error("{0}")]
    HyperError(#[from] hyper_util::client::legacy::Error),

    #[error("{to:?}")]
    Redirected {
        to: String,
    },

    #[error("{uri:?}")]
    LocationMissing {
        uri: String,
    },

    #[error("{0}")]
    StringConvertError(#[from] ToStrError),

    #[error("Too many redirects")]
    TooManyRedirects,

    #[error("No content")]
    NoContent {
        status_code: StatusCode,
    },
}

// Implements simple http1&2 GET http client wrapper.
/// Wraps [hyper] [HttpsConnector] with [aws_lc_rs] [rustls] provider.
pub struct Client {
    https_connector: HttpsConnector<HttpConnector>,
}

impl Client {
    /// Creates a new [hyper] http client wrapper.
    pub(crate) fn new() -> Client {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

        let mut root_cert_store = rustls::RootCertStore::empty();
        root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        let config = rustls::ClientConfig::builder()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();

        Client {
            https_connector: hyper_rustls::HttpsConnectorBuilder::new()
                .with_tls_config(config)
                .https_only()
                .enable_all_versions()
                .build()
        }
    }

    async fn get_without_redirects(&self, url: &str) -> Result<Bytes, ClientError> {
        let uri = url.parse::<Uri>()?;

        let req: Request<Empty<Bytes>> = hyper::Request::builder()
            .uri(uri)
            .method(Method::GET)
            // NOTE: Causes PROTOCOL_ERROR on http2, [answered here](https://github.com/hyperium/hyper/discussions/3676#discussioncomment-9570313)
            // .header(hyper::header::HOST, authority.as_str())
            .body(Empty::new())?;

        let client = HyperClient::builder(TokioExecutor::new())
            .build(self.https_connector.clone());

        let res = client.request(req).await?;

        match res.status() {
            StatusCode::OK => Ok(res.into_body().collect().await?.to_bytes()),
            status if status.is_redirection() => {
                let loc = res.headers().get(hyper::header::LOCATION).ok_or_else(||
                    ClientError::LocationMissing { uri: url.to_string() }
                )?;
                Err(ClientError::Redirected { to: loc.to_str()?.to_string() })
            }
            _ => Err(ClientError::NoContent { status_code: res.status() }),
        }
    }


    const MAX_REDIRECTS: usize = 10;


    /// Performs HTTP GET request, following redirects
    pub async fn get(&self, url: &str) -> Result<Bytes, ClientError> {
        let mut resolved_url = url;
        let mut res = self.get_without_redirects(resolved_url).await;

        for _ in 0..=Self::MAX_REDIRECTS {
            match res.as_ref().err() {
                Some(ClientError::Redirected { to }) => {
                    resolved_url = to.as_str();
                }

                _ => return res
            }

            res = self.get_without_redirects(resolved_url).await;
        }

        Err(ClientError::TooManyRedirects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // It should be able to open google.com, follow redirects and get content
    #[tokio::test]
    async fn client_should_get() {
        assert!(Client::new().get("https://google.com").await.unwrap().len() > 0);
    }
}
