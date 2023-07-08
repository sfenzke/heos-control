use std::{fmt, error::Error};
use ssdp_client::Error as DiscoveryError;

#[derive(Debug)]
pub enum HeosError {
    Discover(DiscoveryError),
    Connect(std::io::Error),
    NoDevices(),
    QuerryError(String)
}

impl fmt::Display for HeosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeosError::Discover(err) => write!(f, "Discovery Error: {}", &err),
            HeosError::Connect(err) => write!(f, "Connection Error: {}", &err),
            HeosError::NoDevices() => write!(f, "No HEOS capable devices found"),
            HeosError::QuerryError(msg) => write!(f, "QuerryError: {}", msg)
        }
    }
}

impl Error for HeosError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
       match self {
        HeosError::Discover(err) => Some(err),
        HeosError::Connect(err) => Some(err),
        _ => None
       }
    }
}

impl From<DiscoveryError> for HeosError {
    fn from(error: DiscoveryError) -> Self {
        Self::Discover(error)
    }
}

impl From<std::io::Error> for HeosError {
    fn from(error: std::io::Error) -> Self {
        Self::Connect(error)
    }
}