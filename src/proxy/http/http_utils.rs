use crate::logging;

use std::io::Write;
use std::net::{Shutdown, TcpStream};

/// HTTP/1.1 responses:
static HTTP_AUTH: &[u8] = "HTTP/1.1 401 Unauthorized\r\nConnection: close\r\n\r\n".as_bytes();
static HTTP_FORBIDDEN: &[u8] = "HTTP/1.1 403 Forbidden\r\nConnection: close\r\n\r\n".as_bytes();
static HTTP_PROXY_AUTH: &[u8] =
    "HTTP/1.1 407 Proxy Authentication Required\r\nProxy-Authenticate: Basic\r\n\r\n".as_bytes();
static HTTPS_NOT_SUPPORTED: &[u8] = "HTTP/1.1 400 Bad Request\r\nConnection: close\r\nContent-Length: 28\r\n\r\nProxy does not support https".as_bytes();
static HTTP_STATUS_OK: &[u8] = "HTTP/1.1 200 OK\r\nProxy-Connection: keep-alive\r\n\r\n".as_bytes();

pub fn unauthorized(stream: &mut TcpStream, logfile: String) {
    let logflag = logfile.len() > 0;

    if let Err(e) = stream.write(HTTP_AUTH) {
        if logflag {
            logging::log_message(
                format!("[-]: Failed to write to TCP stream: {}", e).as_str(),
                logfile.as_str(),
            );
        }
        logging::print_error(format!("Failed to write to TCP stream: {}", e).as_str());
    }

    if let Err(e) = stream.shutdown(Shutdown::Both) {
        if logflag {
            logging::log_message(
                format!("[-]: Failed to shutdown TCP stream: {}", e).as_str(),
                logfile.as_str(),
            );
        }
        logging::print_error(format!("Failed to shutdown TCP stream: {}", e).as_str());
    }
}

pub fn forbidden(stream: &mut TcpStream, logfile: String) {
    let logflag = logfile.len() > 0;

    if let Err(e) = stream.write(HTTP_FORBIDDEN) {
        if logflag {
            logging::log_message(
                format!("[-]: Failed to write to TCP stream: {}", e).as_str(),
                logfile.as_str(),
            );
        }
        logging::print_error(format!("Failed to write to TCP stream: {}", e).as_str());
    }

    if let Err(e) = stream.shutdown(Shutdown::Both) {
        if logflag {
            logging::log_message(
                format!("[-]: Failed to shutdown TCP stream: {}", e).as_str(),
                logfile.as_str(),
            );
        }
        logging::print_error(format!("Failed to shutdown TCP stream: {}", e).as_str());
    }
}

pub fn proxy_auth(stream: &mut TcpStream, logfile: String) {
    let logflag = logfile.len() > 0;

    if let Err(e) = stream.write(HTTP_PROXY_AUTH) {
        if logflag {
            logging::log_message(
                format!("[-]: Failed to write to TCP stream: {}", e).as_str(),
                logfile.as_str(),
            );
        }
        logging::print_error(format!("Failed to write to TCP stream: {}", e).as_str());
    }

    if let Err(e) = stream.shutdown(Shutdown::Both) {
        if logflag {
            logging::log_message(
                format!("[-]: Failed to shutdown TCP stream: {}", e).as_str(),
                logfile.as_str(),
            );
        }
        logging::print_error(format!("Failed to shutdown TCP stream: {}", e).as_str());
    }
}

pub fn https_not_supported(stream: &mut TcpStream, logfile: String) {
    let logflag = logfile.len() > 0;

    if let Err(e) = stream.write(HTTPS_NOT_SUPPORTED) {
        if logflag {
            logging::log_message(
                format!("[-]: Failed to write to TCP stream: {}", e).as_str(),
                logfile.as_str(),
            );
        }
        logging::print_error(format!("Failed to write to TCP stream: {}", e).as_str());
    }

    if let Err(e) = stream.shutdown(Shutdown::Both) {
        if logflag {
            logging::log_message(
                format!("[-]: Failed to shutdown TCP stream: {}", e).as_str(),
                logfile.as_str(),
            );
        }
        logging::print_error(format!("Failed to shutdown TCP stream: {}", e).as_str());
    }
}

pub fn http_status_ok(stream: &mut TcpStream, logfile: String) {
    let logflag = logfile.len() > 0;

    if let Err(e) = stream.write(HTTP_STATUS_OK) {
        if logflag {
            logging::log_message(
                format!("[-]: Failed to write to TCP stream: {}", e).as_str(),
                logfile.as_str(),
            );
        }
        logging::print_error(format!("Failed to write to TCP stream: {}", e).as_str());
    }

    if let Err(e) = stream.shutdown(Shutdown::Both) {
        if logflag {
            logging::log_message(
                format!("[-]: Failed to shutdown TCP stream: {}", e).as_str(),
                logfile.as_str(),
            );
        }
        logging::print_error(format!("Failed to shutdown TCP stream: {}", e).as_str());
    }
}
