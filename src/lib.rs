//! A Rust client for [`block-dn`](https://github.com/guggero/block-dn#).
#![warn(missing_docs)]
use core::fmt;
use std::time::Duration;

use models::{Html, ServerStatus};

/// Data models for server queries and responses.
pub mod models;

/// Errors that may occur when querying a client.
#[derive(Debug)]
pub enum Error {
    /// Underlying HTTPs request failed.
    Request(bitreq::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Request(e) => write!(f, "request error {e}"),
        }
    }
}

impl From<bitreq::Error> for Error {
    fn from(value: bitreq::Error) -> Self {
        Self::Request(value)
    }
}

/// An endpoint for a `block-dn` server.
#[derive(Debug, Clone)]
pub struct Endpoint<'e>(&'e str);

impl<'e> Endpoint<'e> {
    /// The original `block-dn` server hosted at `block-dn.org`.
    pub const BLOCKDNORG: Self = Self("https://block-dn.org");
    /// Taproot-specific filters hosted by `2140.dev`.
    pub const TAPROOTDN: Self = Self("https://taprootdn.xyz");

    /// Use your self-hosted `block-dn` instance.
    pub fn from_custom_domain(other: &'static str) -> Self {
        Self(other)
    }

    /// Append a route to the endpoint.
    fn append_route(&self, hook: &str) -> String {
        format!("{}/{hook}", self.0)
    }
}

/// Build a new client to query data for.
#[derive(Debug)]
pub struct Builder<'e> {
    endpoint: Endpoint<'e>,
    timeout: Duration,
}

impl<'e> Builder<'e> {
    /// Create a new builder [`ClientBuilder`].
    pub fn new() -> Self {
        Self {
            endpoint: Endpoint::BLOCKDNORG,
            timeout: Duration::from_secs(1),
        }
    }

    /// Set the timeout the server has to respond.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Add an endpoint to query.
    pub fn endpoint(mut self, endpoint: Endpoint<'e>) -> Self {
        self.endpoint = endpoint;
        self
    }

    /// Build a [`Client`] from the configuration.
    pub fn build(self) -> Client<'e> {
        Client {
            endpoint: self.endpoint,
            timeout: self.timeout,
        }
    }
}

impl<'e> Default for Builder<'e> {
    fn default() -> Self {
        Self::new()
    }
}

/// A client to request block data.
#[derive(Debug)]
pub struct Client<'e> {
    endpoint: Endpoint<'e>,
    timeout: Duration,
}

impl<'e> Client<'e> {
    /// Return the root HTML of the server.
    pub fn index_html(&self) -> Result<Html, Error> {
        let response = bitreq::get(self.endpoint.0)
            .with_timeout(self.timeout.as_secs())
            .send()?;
        let html = response.as_str()?;
        Ok(Html(html.to_string()))
    }

    /// Get the status of the server.
    pub fn status(&self) -> Result<ServerStatus, Error> {
        let status = bitreq::get(self.endpoint.append_route("status"))
            .with_timeout(self.timeout.as_secs())
            .send()?;
        Ok(status.json::<ServerStatus>()?)
    }
}
