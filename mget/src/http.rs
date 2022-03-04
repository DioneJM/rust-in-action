use std::fmt;
use std::fmt::Formatter;
use std::net::IpAddr;
use std::str::Utf8Error;
use smoltcp::Error;
use smoltcp::wire::EthernetAddress;
use url::Url;

#[derive(Debug)]
enum HttpState {
    Connect,
    Request,
    Response
}

#[derive(Debug)]
pub enum UpstreamError {
    Network(smoltcp::Error),
    InvalidUrl,
    Content(std::str::Utf8Error)
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<smoltcp::Error> for UpstreamError {
    fn from(error: smoltcp::Error) -> Self {
        UpstreamError::Network(error)
    }
}

impl From<std::str::Utf8Error> for UpstreamError {
    fn from(error: Utf8Error) -> Self {
        UpstreamError::Content(error)
    }
}

fn random_port() -> u16 {
    49_152 + rand::random::<u16>() % 16384
}

pub struct TapInterface;

pub fn get(
    tap: TapInterface, // This struct only exists for smoltcp binaries compiled for linux
    mac: EthernetAddress,
    addr: IpAddr,
    url: Url
) -> Result<(), UpstreamError> {
    Ok(())
}