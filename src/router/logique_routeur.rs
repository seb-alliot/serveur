use crate::http::_struct::{Method, Request, Response, StatusCode, Version};
use crate::http::response::html_response;
use crate::router::_struct_routeur::{Data, Route, Router};

fn home(_request: &Request, state: &Data) -> Response {
    let body = format!(
        "<html><head><style>{}</style></head><body><h1>Bienvenue sur la page d'accueil !</h1></body></html>",
        state.style_html
    );
    html_response(Version::HTTP11, StatusCode::OK, body)
}

fn about(_request: &Request, _state: &Data) -> Response {
    html_response(
        Version::HTTP11,
        StatusCode::OK,
        "<html><body><h1>À propos de nous</h1></body></html>".to_string(),
    )
}

// Construit la table de routes une seule fois (au démarrage, cf. main.rs) —
// jamais à chaque requête, ce qui gaspillerait une allocation de Vec par
// connexion pour un contenu qui ne change jamais.
pub fn build_router() -> Router {
    Router::new(vec![
        Route {
            method: Method::GET,
            path: "/",
            handler: home,
        },
        Route {
            method: Method::GET,
            path: "/about",
            handler: about,
        },
    ])
}
