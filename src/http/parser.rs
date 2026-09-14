use super::_struct::{Body, Header, Method, ParseError, Request, Version};

// Tailles maximales pour les différentes parties de la requête HTTP
const MAX_TOTAL_REQUEST_SIZE: usize = 8192; // 8 Ko
const MAX_REQUEST_LINE_SIZE: usize = 2048; // 2 Ko
const MAX_METHOD_LENGTH: usize = 10; // ex: "GET", "POST"
const MAX_PATH_LENGTH: usize = 2048; // 2 Ko
const MAX_VERSION_LENGTH: usize = 10; // ex: "HTTP/1.1"
const MAX_BODY_SIZE: usize = 8192; // 8 Ko

pub fn parse_request(buffer: &str) -> Result<Request<'_>, ParseError> {
    // Guard: taille totale raisonnable
    if buffer.len() > MAX_TOTAL_REQUEST_SIZE {
        return Err(ParseError::RequestTooLarge);
    }

    let mut lines = buffer.lines();
    let first_line = lines.next().ok_or(ParseError::EmptyRequest)?;

    // Guard: taille de la request line
    if first_line.len() > MAX_REQUEST_LINE_SIZE {
        return Err(ParseError::RequestLineTooLong);
    }

    // Parsing de la request line
    let mut parts = first_line.split_whitespace();
    let method_str = parts.next().ok_or(ParseError::MissingMethod)?;
    let path = parts.next().ok_or(ParseError::MissingPath)?;
    let version_str = parts.next().ok_or(ParseError::MissingVersion)?;

    if parts.next().is_some() {
        return Err(ParseError::TooManyPartsInRequestLine);
    }

    // Guards individuels sur chaque partie
    if method_str.len() > MAX_METHOD_LENGTH {
        return Err(ParseError::MethodTooLong);
    }
    if path.len() > MAX_PATH_LENGTH {
        return Err(ParseError::PathTooLong);
    }
    if version_str.len() > MAX_VERSION_LENGTH {
        return Err(ParseError::VersionTooLong);
    }

    // Method/Version : FromStr renvoie directement ParseError, ? propage l'échec.
    let method = method_str.parse::<Method>()?;
    let version = version_str.parse::<Version>()?;

    // Headers : on s'arrête à la première ligne vide (fin des headers, début du body).
    let headers: Vec<Header> = lines
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let (name, value) = line.split_once(':')?;
            Some(Header {
                name: name.trim(),
                value: value.trim(),
            })
        })
        .collect();

    // Body : tout ce qui suit le séparateur "\r\n\r\n" dans le buffer d'origine.
    let body_start_index = buffer.find("\r\n\r\n").map(|i| i + 4).unwrap_or(buffer.len());
    let body_content = &buffer[body_start_index..];

    if body_content.len() > MAX_BODY_SIZE {
        return Err(ParseError::BodyTooLarge);
    }

    let body = if body_content.is_empty() {
        None
    } else {
        Some(Body {
            content: body_content,
        })
    };

    Ok(Request {
        method,
        path,
        version,
        headers,
        body,
    })
}
