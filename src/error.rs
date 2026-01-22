use core::fmt;

/// Errors that may occur when querying a client.
#[derive(Debug)]
pub enum Error {
    /// A consensus error was encodered when decoding the response.
    Decoder(bitcoin::consensus::encode::Error),
    /// Underlying HTTPs request failed.
    Request(bitreq::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Decoder(e) => write!(f, "consensus error {e}"),
            Error::Request(e) => write!(f, "request error {e}"),
        }
    }
}

impl From<bitreq::Error> for Error {
    fn from(value: bitreq::Error) -> Self {
        Self::Request(value)
    }
}

impl From<bitcoin::consensus::encode::Error> for Error {
    fn from(value: bitcoin::consensus::encode::Error) -> Self {
        Self::Decoder(value)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Decoder(d) => Some(d),
            Self::Request(r) => Some(r),
        }
    }
}
