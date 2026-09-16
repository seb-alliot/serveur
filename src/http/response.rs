use super::_struct::{ParseError, Response, StatusCode, Version};

// Sérialise un Response en octets HTTP prêts à écrire sur le socket :
// status-line + headers + ligne vide + body. Aucune décision métier ici,
// juste de la mise en forme — comme parser.rs ne fait que du découpage.
pub fn build_response(response: &Response) -> String {
    let reason = response.status_code.response_string().unwrap_or("Unknown Status");

    let mut output = format!(
        "{} {} {reason}\r\n",
        response.version.as_str(),
        response.status_code.as_u16()
    );

    for (name, value) in &response.headers {
        output.push_str(name);
        output.push_str(": ");
        output.push_str(value);
        output.push_str("\r\n");
    }
    output.push_str("\r\n");

    if let Some(body) = &response.body {
        output.push_str(body);
    }

    output
}

// Associe chaque échec de parsing au code HTTP le plus proche de sa cause.
pub fn status_for_error(error: &ParseError) -> StatusCode {
    match error {
        ParseError::UnknownMethod => StatusCode::NOT_IMPLEMENTED,
        ParseError::UnknownVersion => StatusCode::HTTP_VERSION_NOT_SUPPORTED,
        _ => StatusCode::BAD_REQUEST,
    }
}

// Construit une réponse HTML minimale avec les headers communs déjà posés
// (Content-Length calculé, Content-Type, Connection) — évite de les répéter
// à chaque route/cas d'erreur.
pub fn html_response(version: Version, status_code: StatusCode, body: String) -> Response {
    Response {
        version,
        headers: vec![
            ("Content-Length".to_string(), body.len().to_string()),
            ("Content-Type".to_string(), "text/html; charset=utf-8".to_string()),
            ("Connection".to_string(), "close".to_string()),
        ],
        body: Some(body),
        status_code,
    }
}

// Le "fallback" du routeur : appelé quand AUCUNE route enregistrée ne correspond
// au couple (method, path) reçu. Tout routeur réel (axum a `.fallback()`, Runique
// a une vue 404 par défaut) a besoin d'un point de sortie explicite pour ce cas —
// sans lui, un chemin inconnu n'aurait tout simplement pas de réponse définie.
pub fn not_found(version: Version) -> Response {
    html_response(
        version,
        StatusCode::NOT_FOUND,
        "<html><body><h1>404 Not Found</h1></body></html>".to_string(),
    )
}
