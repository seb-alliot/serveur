#[derive(Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub struct Request<'a> {
     pub method: Method,
     pub path: &'a str,
     pub version: Version,
     pub headers: Vec<Header<'a>>,
     pub body: Option<&'a str>,
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
     MissingMethode,
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
        pub const MISSING_VERSION: StatusCode = StatusCode(505);

        pub fn response_string(&self) -> Option<&'static str> {
            match *self {
                StatusCode::OK => Some("OK"),
                StatusCode::BAD_REQUEST => Some("Bad Request"),
                StatusCode::NOT_FOUND => Some("Not Found"),
                StatusCode::INTERNAL_SERVER_ERROR => Some("Internal Server Error"),
                StatusCode::MISSING_METHODE => Some("Not Implemented"),
                StatusCode::BAD_GATEWAY => Some("Bad Gateway"),
                StatusCode::SERVICE_UNAVAILABLE => Some("Service Unavailable"),
                StatusCode::MISSING_VERSION => Some("HTTP Version Not Supported"),
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
    MissingVersion,
}

impl std::str::FromStr for Version {
    type Err = std::convert::Infallible; // ou un type d'erreur si tu veux gérer l'échec proprement

    fn from_str(version: &str) -> Result<Self, Self::Err> {
        Ok(match version {
            "HTTP/1.0" => Version::HTTP10,
            "HTTP/1.1" => Version::HTTP11,
            "HTTP/2.0" => Version::HTTP20,
            "HTTP/3.0" => Version::HTTP30,
            _ => Version::MissingVersion,
        })
    }
}

 #[derive(Debug, Clone, PartialEq, Eq, Hash)]
 #[allow(dead_code)]
pub struct Header<'a> {
    pub name: &'a str,
    pub value: &'a str,
}
