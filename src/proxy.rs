use std::net;
use std::io;

/// ProxyError represents an error that may occur while working with and/or 
/// making a http proxy.
#[derive(Debug)]
pub enum ProxyError {
    CouldNotSetTTL,
    CouldNotGetTTL,
    CouldNotBindPort(io::Error),
    ConnectionError(io::Error),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct HTTPProxy {
    pub port: u16,
    listener: net::TcpListener,
}

/// proxy::new creates a new HTTPProxy instance and returns it.
///
/// Arguments:
/// - port: specifies the port to be used by the http proxy.
pub fn new(port: u16) -> Result<HTTPProxy, ProxyError> {
    match net::TcpListener::bind(("127.0.0.1", port)) {
        Ok(listener) => Ok(HTTPProxy { port, listener }),
        Err(e) => Err(ProxyError::CouldNotBindPort(e)),
    }
}

impl HTTPProxy {
    pub fn handle_connection(&self) -> Result<
        (net::TcpStream, net::SocketAddr),
        ProxyError> {
        match self.listener.accept() {
            Ok((strm, sock)) => Ok((strm, sock)),
            Err(e) => Err(ProxyError::ConnectionError(e)),
        }
    }

    /// set_ttl wraps the net::TcpListener.set_ttl method provided in the 
    /// standard library and allows the user to set a specific ttl value for
    /// the proxy's listener if necessay.
    ///
    /// Arguments:
    /// - ttl: A u32 value specifying the TTL duration for packets
    pub fn set_ttl(&self, ttl: u32) -> Result<(), ProxyError> {
        match self.listener.set_ttl(ttl) {
            Ok(_) => Ok(()),
            Err(_) => Err(ProxyError::CouldNotSetTTL), 
        }
    }

    /// ttl wraps the net::TcpListener.ttl() method provided in the standard 
    /// library. It returns the IP_TTL value for the corresponding proxy
    /// socket.
    pub fn ttl(&self) -> Result<u32, ProxyError> {
        match self.listener.ttl() {
            Ok(t) => Ok(t),
            Err(_) => Err(ProxyError::CouldNotGetTTL),
        }
    }
}
