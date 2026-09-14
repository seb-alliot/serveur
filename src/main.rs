use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

mod http;
use http::parser::get_path;

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

    let listener = TcpListener::bind("127.0.0.1:3000")?;
    tracing::info!("écoute sur http://127.0.0.1:3000");

    for stream in listener.incoming() {
        let stream = stream?;
        // une seule connexion à la fois pour l'instant : pas de thread, pas d'async.
        // objectif : voir ce qui transite sur le socket avant d'introduire de la concurrence.
        handle_connection(stream);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    // tampon fixe côté pile : on lit au maximum 4096 octets d'un coup.
    // si la requête (headers + body) dépasse cette taille, il faudrait
    // boucler sur plusieurs read() — pas géré pour l'instant.
    let mut buffer = [0u8; 4096];
    tracing::info!("attente d'une requête HTTP sur le socket...");

    // read() remplit `buffer` et renvoie soit le nombre d'octets lus (Ok(n)),
    // soit une erreur si le socket a un problème (connexion coupée, etc.).
    let n = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            // on ne peut rien faire de plus : on log et on abandonne cette connexion.
            tracing::error!("erreur de lecture: {e}");
            return;
        }
    };

    let raw_request = String::from_utf8_lossy(&buffer[..n]);
    tracing::debug!("requête brute:\n{raw_request}");
    // on construis la reponse HTTP à renvoyer au client. Pour l'instant, on ne fait que renvoyer un simple HTML.
    // On peux renvoyer tout type de contenu, mais il faut alors adapter le header Content-Type en conséquence.
    // On peut aussi renvoyer un fichier statique (HTML, CSS, JS, image...) en lisant son contenu depuis le disque.
    let style_html = std::fs::read_to_string("static/main.css").unwrap();
    // Ajout de la balise style pour inclure le CSS dans le HTML.
    let response_body: String = format!(
        "<html><head><style>{style_html}</style></head><body><h1>Bonjour Sébastien</h1></body></html>"
    );
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\nConnection: close\r\n\r\n{response_body}",
        response_body.len()
    );
    tracing::debug!("taille de la réponse: {} octets", response_body.len());

    if let Err(e) = stream.write_all(response.as_bytes()) {
        tracing::error!("erreur d'écriture: {e}");
    }
    let path = get_path(&raw_request);
    println!("path: {path}");
}
