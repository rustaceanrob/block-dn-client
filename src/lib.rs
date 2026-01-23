//! A Rust client for [`block-dn`](https://github.com/guggero/block-dn#).
#![warn(missing_docs)]
use std::{borrow::Cow, io::Cursor, net::SocketAddr};

use bitcoin::{Block, BlockHash, bip158::BlockFilter, block::Header, consensus::Decodable};
use models::{Html, ServerStatus, TapTweaks};

/// Errors that may occur when querying.
pub mod error;
/// Data models for server queries and responses.
pub mod models;

use crate::error::Error;

/// An endpoint for a `block-dn` server.
#[derive(Debug, Clone)]
pub struct Endpoint<'e>(Cow<'e, str>);

impl<'e> Endpoint<'e> {
    /// The original `block-dn` server hosted at `block-dn.org`.
    pub const BLOCK_DN_ORG: Self = Self(Cow::Borrowed("https://block-dn.org"));
    // Taproot-specific filters hosted by `2140.dev`.
    // pub const TAPROOT_DN: Self = Self(Cow::Borrowed("https://taprootdn.xyz"));
    /// Local host at port 8080.
    pub const LOCAL_HOST: Self = Self(Cow::Borrowed("https://127.0.0.1:8080"));

    /// Use your self-hosted `block-dn` instance.
    pub fn from_custom_domain(other: &'static str) -> Self {
        Self(Cow::Borrowed(other))
    }

    /// Use a static IP address.
    pub fn from_socket_address(other: SocketAddr) -> Self {
        let address = other.to_string();
        Self(Cow::Owned(format!("https://{address}")))
    }

    /// Append a route to the endpoint.
    fn append_route(&self, hook: impl AsRef<str>) -> String {
        format!("{}/{}", self.0, hook.as_ref())
    }
}

/// The response timeout permitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timeout(u64);

impl Timeout {
    /// Build a timeout from number of seconds.
    pub const fn from_seconds(seconds: u64) -> Self {
        Self(seconds)
    }
}

impl Default for Timeout {
    fn default() -> Self {
        Self(1)
    }
}

/// Build a new client to query data for.
#[derive(Debug)]
pub struct Builder<'e> {
    endpoint: Endpoint<'e>,
    timeout: Timeout,
}

impl<'e> Builder<'e> {
    /// Create a new builder [`ClientBuilder`].
    pub fn new() -> Self {
        Self {
            endpoint: Endpoint::BLOCK_DN_ORG,
            timeout: Timeout::default(),
        }
    }

    /// Set the timeout the server has to respond.
    pub fn timeout(mut self, timeout: Timeout) -> Self {
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
    timeout: Timeout,
}

impl<'e> Client<'e> {
    const EXPECTED_HEADER_LIST_SIZE: usize = 100_000;
    /// Return the root HTML of the server.
    pub fn index_html(&self) -> Result<Html, Error> {
        let response = bitreq::get(self.endpoint.0.to_string())
            .with_timeout(self.timeout.0)
            .send()?;
        let html = response.as_str()?;
        Ok(Html(html.to_string()))
    }

    /// Get the status of the server. See [`ServerStatus`] for the response structure.
    pub fn status(&self) -> Result<ServerStatus, Error> {
        let status = bitreq::get(self.endpoint.append_route("status"))
            .with_timeout(self.timeout.0)
            .send()?;
        Ok(status.json::<ServerStatus>()?)
    }

    /// Return up to 100,000 block headers starting from the specified height.
    pub fn block_headers(&self, start_height: u32) -> Result<Vec<Header>, Error> {
        let route = self
            .endpoint
            .append_route(format!("headers/{start_height}"));
        let response = bitreq::get(route).with_timeout(self.timeout.0).send()?;
        let mut headers = Vec::with_capacity(Self::EXPECTED_HEADER_LIST_SIZE * 80);
        for chunk in response.as_bytes().chunks_exact(80) {
            headers.push(bitcoin::consensus::deserialize::<Header>(chunk)?);
        }
        Ok(headers)
    }

    /// Return up to 2,000 compact block filters starting from the specified height.
    pub fn filters(&self, start_height: u32) -> Result<Vec<BlockFilter>, Error> {
        let route = self
            .endpoint
            .append_route(format!("filters/{start_height}"));
        let response = bitreq::get(route).with_timeout(self.timeout.0).send()?;
        let mut cursor = Cursor::new(response.into_bytes());
        let mut filters = Vec::new();
        while let Ok(bytes) = Vec::<u8>::consensus_decode_from_finite_reader(&mut cursor) {
            filters.push(BlockFilter::new(&bytes));
        }
        Ok(filters)
    }

    /// Return up to 2,000 blocks of BIP-352 partial secrets (key tweaks).
    pub fn tweaks(&self, start_height: u32) -> Result<TapTweaks, Error> {
        let route = self
            .endpoint
            .append_route(format!("sp/tweak-data/{start_height}"));
        let response = bitreq::get(route).with_timeout(self.timeout.0).send()?;
        Ok(response.json::<TapTweaks>()?)
    }

    /// Fetch the block by its hash.
    pub fn block(&self, block_hash: BlockHash) -> Result<Block, Error> {
        let route = self.endpoint.append_route(format!("block/{block_hash}"));
        let response = bitreq::get(route).with_timeout(self.timeout.0).send()?;
        let block = bitcoin::consensus::deserialize::<Block>(response.as_bytes())?;
        Ok(block)
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use crate::Endpoint;

    #[test]
    fn test_endpoint() {
        let google = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 8080);
        let endpoint = Endpoint::from_socket_address(google);
        let filters_route = endpoint.append_route("filters/0");
        assert_eq!(filters_route.as_str(), "https://8.8.8.8:8080/filters/0");
    }
}
