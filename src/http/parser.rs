use super::_struct::{Method, Request, Response, StatusCode, Version, Header};

pub fn get_path(buffer: &str) -> &str {
    let first_line = buffer.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    parts.get(1).copied().unwrap_or("")
}

 #[allow(dead_code)]
pub fn get_method(buffer: &str) -> Method {
    let first_line = buffer.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    match parts.get(0).copied() {
        Some("GET") => Method::GET,
        Some("POST") => Method::POST,
        Some("PUT") => Method::PUT,
        Some("DELETE") => Method::DELETE,
        Some("PATCH") => Method::PATCH,
        Some("OPTIONS") => Method::OPTIONS,
        Some("HEAD") => Method::HEAD,
        Some("CONNECT") => Method::CONNECT,
        Some("TRACE") => Method::TRACE,
        _ => {
            Method::MissingMethode
        }
    }
}

 #[allow(dead_code)]
pub fn get_version(buffer: &str) -> Version {
    let first_line = buffer.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    parts.get(2).copied().unwrap_or("").parse().unwrap()
    // .parse::<Version>() utilise FromStr ; Infallible garantit que unwrap() ne panique jamais
}

#[allow(dead_code)]
pub fn get_headers(buffer: &str) -> Vec<Header<'_>> {
    let mut headers = Vec::new();
    for line in buffer.lines().skip(1) {
        if line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            headers.push(Header {
                name: name.trim(),
                value: value.trim(),
            });
        }
    }
    headers
}