use std::io::{Read, Write};
use std::net::TcpStream;

use super::http::_struct::Version;
use super::http::parser::parse_request;
use super::http::response::{build_response, html_response, not_found, status_for_error};
use crate::router::_struct_routeur::{Data, Router};

pub fn handle_connection(mut stream: TcpStream, style_html: &str, profiling: bool, router: &Router) {
    // Regroupe tout ce que les handlers peuvent lire en dehors de la requête —
    // ici juste le CSS déjà chargé, potentiellement plus tard (config, etc.).
    let state = Data { style_html };
    // Afficher le temps de reponse pour chaque requête HTTP reçue.
    let start_time = std::time::Instant::now();
    // tampon fixe côté pile : on lit au maximum 4096 octets d'un coup.
    // si la requête (headers + body) dépasse cette taille, il faudrait
    // boucler sur plusieurs read() — pas géré pour l'instant.
    let mut buffer = [0u8; 4096];
    tracing::info!("attente d'une requête HTTP sur le socket...");

    // read() remplit `buffer` et renvoie soit le nombre d'octets lus (Ok(n)),
    // soit une erreur si le socket a un problème (connexion coupée, etc.).
    let read_start = std::time::Instant::now();
    let n = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            // on ne peut rien faire de plus : on log et on abandonne cette connexion.
            tracing::error!("erreur de lecture: {e}");
            return;
        }
    };
    if profiling {
        tracing::debug!("étape lecture socket: {:?}", read_start.elapsed());
    }

    // Décodage UTF-8 + log de la requête brute : untimed avant, ça masquait du coût.
    let decode_start = std::time::Instant::now();
    let raw_request = String::from_utf8_lossy(&buffer[..n]);
    tracing::debug!("requête brute:\n{raw_request}");
    if profiling {
        tracing::debug!("étape décodage + log requête brute: {:?}", decode_start.elapsed());
    }

    // Parsing avant construction de la réponse : c'est lui qui décide maintenant
    // quoi répondre (path/method), donc il doit passer en premier.
    let parse_start = std::time::Instant::now();
    let parsed = parse_request(&raw_request);
    match &parsed {
        Ok(request) => tracing::debug!("requête parsée:\n{request:#?}"),
        Err(e) => tracing::warn!("échec du parsing: {e:?}"),
    }
    if profiling {
        tracing::debug!("étape parsing requête (log inclus): {:?}", parse_start.elapsed());
    }

    let build_start = std::time::Instant::now();
    let response = match parsed {
        // Cas nominal : la requête a pu être parsée. On route sur le couple
        // (method, path) — c'est la vraie question qu'un routeur se pose :
        // "quelle action correspond à CETTE combinaison précise ?", pas
        // "quel est le path ?" et "quelle est la méthode ?" séparément.
        Ok(request) => match router.dispatch(&request, &state) {
            Some(response) => response,
            // Fallback : AUCUNE route ci-dessus ne correspond — que ce soit
            // parce que le path est inconnu ("/xyz") OU parce que la méthode
            // ne convient pas pour un path par ailleurs valide (ex: POST /).
            // Un vrai routeur (axum, Runique) a toujours un point de sortie
            // explicite pour ce cas — jamais de requête sans réponse définie.
            None => not_found(request.version),
        },
        // Cas d'échec : la requête elle-même n'a pas pu être parsée (méthode
        // inconnue, version invalide, requête trop grosse...). On ne peut même
        // pas savoir quelle route était visée — le mapping ParseError → StatusCode
        // (status_for_error) vit dans response.rs, pas ici : ce fichier décide
        // du routing, response.rs décide de la traduction erreur → code HTTP.
        Err(e) => {
            let status = status_for_error(&e);
            let reason = status.response_string().unwrap_or("Error");
            let body = format!("<html><body><h1>{} {reason}</h1></body></html>", status.as_u16());
            // la version n'est pas toujours connue si le parsing a échoué tôt (ex: ligne vide) ;
            // HTTP/1.1 est une hypothèse par défaut raisonnable, comme le font la plupart des serveurs.
            html_response(Version::HTTP11, status, body)
        }
    };
    let response_bytes = build_response(&response);
    if profiling {
        tracing::debug!("étape construction réponse: {:?}", build_start.elapsed());
    }

    // Logs de la réponse : untimed avant, alors qu'imprimer tout le HTML sur stdout a un coût.
    let response_log_start = std::time::Instant::now();
    tracing::debug!("taille de la réponse: {} octets", response_bytes.len());
    tracing::debug!("réponse envoyée:\n{response_bytes}");
    if profiling {
        tracing::debug!("étape logs de réponse: {:?}", response_log_start.elapsed());
    }

    let write_start = std::time::Instant::now();
    if let Err(e) = stream.write_all(response_bytes.as_bytes()) {
        tracing::error!("erreur d'écriture: {e}");
    }
    if profiling {
        tracing::debug!("étape écriture socket: {:?}", write_start.elapsed());
    }

    let end_time = std::time::Instant::now();
    tracing::info!("temps de réponse: {:?}", end_time - start_time);
}