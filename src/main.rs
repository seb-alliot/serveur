use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod http;
use http::parser::parse_request;

fn main() -> std::io::Result<()> {
    // installe le subscriber par défaut : sans lui, les macros tracing (info!, error!...)
    // n'affichent rien nulle part, elles ne font qu'émettre des évènements dans le vide.
    // niveau par défaut "debug" (comme avant avec println!/eprintln! qui affichaient tout) ;
    // RUST_LOG reste prioritaire si défini (ex: RUST_LOG=warn cargo run).
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug")),
        )
        .init();
    let style_html = std::fs::read_to_string("static/main.css")?;

    let listener = TcpListener::bind("127.0.0.1:3000")?;
    // flag lu une seule fois au démarrage : active le détail du chronométrage par étape.
    // usage : SERVEUR_PROFILE=1 cargo run --release
    let profiling = std::env::var("SERVEUR_PROFILE").is_ok();
    tracing::info!("écoute sur http://127.0.0.1:3000");

    for stream in listener.incoming() {
        let stream = stream?;
        // une seule connexion à la fois pour l'instant : pas de thread, pas d'async.
        // objectif : voir ce qui transite sur le socket avant d'introduire de la concurrence.
        handle_connection(stream, profiling, &style_html);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, profiling: bool, style_html: &str) {
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

    let raw_request = String::from_utf8_lossy(&buffer[..n]);
    tracing::debug!("requête brute:\n{raw_request}");
    // on construis la reponse HTTP à renvoyer au client. Pour l'instant, on ne fait que renvoyer un simple HTML.
    // On peux renvoyer tout type de contenu, mais il faut alors adapter le header Content-Type en conséquence.
    // On peut aussi renvoyer un fichier statique (HTML, CSS, JS, image...) en lisant son contenu depuis le disque.
    let css_start = std::time::Instant::now();
    if profiling {
        tracing::debug!("étape lecture CSS (disque): {:?}", css_start.elapsed());
    }

    // Ajout de la balise style pour inclure le CSS dans le HTML.
    let build_start = std::time::Instant::now();
    let response_body: String = format!(
        "<html><head><style>{style_html}</style></head><body><h1>Bonjour Sébastien</h1></body></html>"
    );
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\nConnection: close\r\n\r\n{response_body}",
        response_body.len()
    );
    if profiling {
        tracing::debug!("étape construction réponse: {:?}", build_start.elapsed());
    }
    tracing::debug!("taille de la réponse: {} octets", response_body.len());
    tracing::debug!("réponse envoyée:\n{response}");

    let write_start = std::time::Instant::now();
    if let Err(e) = stream.write_all(response.as_bytes()) {
        tracing::error!("erreur d'écriture: {e}");
    }
    if profiling {
        tracing::debug!("étape écriture socket: {:?}", write_start.elapsed());
    }

    let parse_start = std::time::Instant::now();
    match parse_request(&raw_request) {
        Ok(request) => tracing::debug!("requête parsée:\n{request:#?}"),
        Err(e) => tracing::warn!("échec du parsing: {e:?}"),
    }
    if profiling {
        tracing::debug!("étape parsing requête: {:?}", parse_start.elapsed());
    }

    let end_time = std::time::Instant::now();
    tracing::info!("temps de réponse: {:?}", end_time - start_time);
}
