use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{BufRead, BufReader, Read};

/// HTTPMethod enumerates all the different HTTP request methods
///
/// See RFC-9110 for full specifications
#[derive(Debug, PartialEq, Eq)]
pub enum HTTPMethod {
    GET,
    POST,
    HEAD,
    CONNECT,
    OTHERS(String), // Methods like PATCH and TRACE
}

impl HTTPMethod {
    pub fn parse(method: &str) -> HTTPMethod {
        match method {
            "GET" => HTTPMethod::GET,
            "POST" => HTTPMethod::POST,
            "HEAD" => HTTPMethod::HEAD,
            "CONNECT" => HTTPMethod::CONNECT,
            _ => HTTPMethod::OTHERS(method.to_string()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::GET => String::from("GET"),
            Self::POST => String::from("POST"),
            Self::HEAD => String::from("HEAD"),
            Self::CONNECT => String::from("CONNECT"),
            Self::OTHERS(method) => method.clone(),
        }
    }
}

impl Display for HTTPMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_string())
    }
}

impl Default for HTTPMethod {
    fn default() -> Self {
        HTTPMethod::GET
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum HTTPVersion {
    HTTPv1,
    HTTPv1_1,
    HTTPv2,
    HTTPv3,
}

impl HTTPVersion {
    pub fn parse(version: &str) -> Result<HTTPVersion, String> {
        match version {
            "HTTP/1.0" => Ok(HTTPVersion::HTTPv1),
            "HTTP/1.1" => Ok(HTTPVersion::HTTPv1_1),
            "HTTP/2" => Ok(HTTPVersion::HTTPv2),
            "HTTP/3" => Ok(HTTPVersion::HTTPv3),
            _ => Err(format!("Invalid HTTP version specified: {}", version)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::HTTPv1 => String::from("HTTP/1.0"),
            Self::HTTPv1_1 => String::from("HTTP/1.1"),
            Self::HTTPv2 => String::from("HTTP/2"),
            Self::HTTPv3 => String::from("HTTP/3"),
        }
    }
}

impl Display for HTTPVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_string())
    }
}

impl Default for HTTPVersion {
    fn default() -> Self {
        HTTPVersion::HTTPv1_1
    }
}

/// RequestLine represents a HTTP request's header line:
///         <version> <method> <path>
///
/// Check RFC-9110 for more infomation
#[derive(Debug)]
pub struct RequestLine {
    version: HTTPVersion,
    method: HTTPMethod,
    path: String,
}

pub fn parse_request_header(line: &str) -> Result<RequestLine, String> {
    let line: Vec<_> = line.split(' ').collect();
    if line.len() != 3 {
        return Err("Request line is not correct".to_string());
    }
    let method = HTTPMethod::parse(line[0]);
    let path = line[1];
    let version = HTTPVersion::parse(line[2])?;
    Ok(RequestLine {
        method,
        path: path.to_string().clone(),
        version,
    })
}

#[derive(Debug)]
pub struct ResponseLine {
    pub version: HTTPVersion,
    pub code: u16,
    pub text: String,
}

fn parse_response_header(line: &str) -> Result<ResponseLine, String> {
    let line: Vec<_> = line.splitn(3, ' ').collect();
    if line.len() != 3 {
        return Err("Invalid response line".to_string());
    }
    let version = HTTPVersion::parse(line[0])?;

    let code = match line[1].parse() {
        Ok(res) => res,
        Err(e) => {
            return Err(format!("Error in parsing the response code: {:?}", e));
        }
    };

    let text = line[2];
    Ok(ResponseLine {
        version,
        code,
        text: text.to_string(),
    })
}

#[derive(Debug, Default)]
pub struct Request {
    pub method: HTTPMethod,
    pub path: String,
    pub version: HTTPVersion,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    cache: Vec<u8>,
}

impl Request {
    pub fn to_bytes(&mut self) -> Vec<u8> {
        if self.cache.len() > 0 {
            return self.cache.clone();
        }

        let mut buf: Vec<u8> = Vec::new();

        buf.append(
            &mut format!(
                "{} {} {}\r\n",
                self.method.to_string(),
                self.path,
                self.version.to_string()
            )
            .as_bytes()
            .to_vec(),
        );
        let mut b_headers: Vec<u8> = vec![];
        for (key, value) in self.headers.clone() {
            b_headers.append(&mut format!("{}: {}\r\n", key, value).as_bytes().to_vec());
        }

        buf.append(&mut b_headers);

        self.cache = buf.clone();

        buf
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} \n {:?}",
            self.method, self.path, self.version, self.headers
        )
    }
}

#[derive(Debug, Default)]
pub struct Response {
    pub version: HTTPVersion,
    pub code: u16,
    pub text: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    cache: Vec<u8>,
}

impl Response {
    pub fn to_bytes(&mut self) -> Vec<u8> {
        if self.cache.len() > 0 {
            return self.cache.clone();
        }

        let mut buf: Vec<u8> = Vec::new();

        buf.append(
            &mut format!(
                "{} {} {}\r\n",
                self.version.to_string(),
                self.code,
                self.text
            )
            .as_bytes()
            .to_vec(),
        );
        let mut b_headers: Vec<u8> = vec![];
        for (key, value) in self.headers.clone() {
            b_headers.append(&mut format!("{}: {}\r\n", key, value).as_bytes().to_vec());
        }

        buf.append(&mut b_headers);
        buf.append(&mut self.body);

        self.cache = buf.clone();

        buf
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} \n {:?}",
            self.version, self.code, self.text, self.headers
        )
    }
}
