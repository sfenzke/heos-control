use std::{fmt};
use ssdp_client::Error as DiscoveryError;

pub enum HeosError {
    Discover(DiscoveryError),
    Connect(std::io::Error),
    NoDevices()
}

impl fmt::Display for HeosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeosError::Discover(err) => write!(f, "Discovery Error: {}", &err),
            HeosError::Connect(err) => write!(f, "Connection Error: {}", &err),
            HeosError::NoDevices() => write!(f, "No HEOS capable devices found")
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