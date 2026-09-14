#[derive(Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub struct Request<'a> {
     pub method: Method,
     pub path: &'a str,
     pub version: Version,
     pub headers: Vec<Header<'a>>,
     pub body: Option<Body<'a>>,
 }

 #[derive(Debug, PartialEq, Eq, Hash)]
 #[allow(dead_code)]
 pub enum Method {
     GET,
     POST,
     PUT,
     DELETE,
     PATCH,
     OPTIONS,
     HEAD,
     CONNECT,
     TRACE,
 }

 #[derive(Debug, Clone, PartialEq, Eq, Hash)]
 #[allow(dead_code)]
 pub struct Response {
     pub status_code: StatusCode,
     pub status_message: &'static str,
     pub headers: Vec<(String, String)>,
     pub body: Option<String>,
 }

 #[derive(Debug, Clone, PartialEq, Eq, Hash)]
 #[allow(dead_code)]
 pub struct StatusCode(u16);

  #[allow(dead_code)]
 impl StatusCode {
        pub const OK: StatusCode = StatusCode(200);
        pub const BAD_REQUEST: StatusCode = StatusCode(400);
        pub const NOT_FOUND: StatusCode = StatusCode(404);
        pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode(500);
        pub const MISSING_METHODE: StatusCode = StatusCode(501);
        pub const BAD_GATEWAY: StatusCode = StatusCode(502);
        pub const SERVICE_UNAVAILABLE: StatusCode = StatusCode(503);

        pub fn response_string(&self) -> Option<&'static str> {
            match *self {
                StatusCode::OK => Some("OK"),
                StatusCode::BAD_REQUEST => Some("Bad Request"),
                StatusCode::NOT_FOUND => Some("Not Found"),
                StatusCode::INTERNAL_SERVER_ERROR => Some("Internal Server Error"),
                StatusCode::MISSING_METHODE => Some("Not Implemented"),
                StatusCode::BAD_GATEWAY => Some("Bad Gateway"),
                StatusCode::SERVICE_UNAVAILABLE => Some("Service Unavailable"),
                _ => None,
            }
        }

    }

  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
 #[allow(dead_code)]
pub enum Version {
    HTTP10,
    HTTP11,
    HTTP20,
    HTTP30,
}

impl std::str::FromStr for Version {
    type Err = ParseError;

    fn from_str(version: &str) -> Result<Self, Self::Err> {
        match version {
            "HTTP/1.0" => Ok(Version::HTTP10),
            "HTTP/1.1" => Ok(Version::HTTP11),
            "HTTP/2.0" => Ok(Version::HTTP20),
            "HTTP/3.0" => Ok(Version::HTTP30),
            _ => Err(ParseError::UnknownVersion),
        }
    }
}

 #[derive(Debug, Clone, PartialEq, Eq, Hash)]
 #[allow(dead_code)]
pub struct Header<'a> {
    pub name: &'a str,
    pub value: &'a str,
}


 #[derive(Debug, Clone, PartialEq, Eq, Hash)]
 #[allow(dead_code)]
pub struct Body<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum ParseError {
    RequestTooLarge,
    EmptyRequest,
    RequestLineTooLong,
    MissingMethod,
    MissingPath,
    MissingVersion,
    TooManyPartsInRequestLine,
    MethodTooLong,
    PathTooLong,
    VersionTooLong,
    BodyTooLarge,
    UnknownMethod,
    UnknownVersion,
}

impl std::str::FromStr for Method {
    type Err = ParseError;

    fn from_str(method: &str) -> Result<Self, Self::Err> {
        match method {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "PATCH" => Ok(Method::PATCH),
            "OPTIONS" => Ok(Method::OPTIONS),
            "HEAD" => Ok(Method::HEAD),
            "CONNECT" => Ok(Method::CONNECT),
            "TRACE" => Ok(Method::TRACE),
            _ => Err(ParseError::UnknownMethod),
        }
    }
}