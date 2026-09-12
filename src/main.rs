use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    println!("écoute sur http://127.0.0.1:3000");

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

    // read() remplit `buffer` et renvoie soit le nombre d'octets lus (Ok(n)),
    // soit une erreur si le socket a un problème (connexion coupée, etc.).
    let n = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            // on ne peut rien faire de plus : on log et on abandonne cette connexion.
            eprintln!("erreur de lecture: {e}");
            return;
        }
    };

    let raw_request = String::from_utf8_lossy(&buffer[..n]);
    println!("--- requête brute ---\n{raw_request}");
    // on construis la reponse HTTP à renvoyer au client. Pour l'instant, on ne fait que renvoyer un simple HTML.
    // On peux renvoyer tout type de contenu, mais il faut alors adapter le header Content-Type en conséquence.
    let style_html = "style='display: flex; justify-content: center; color: green; font-size: 3rem; font-family: sans-serif'";
    let response_body = format!("<html><body><h1 {style_html}>Bonjour Sébastien</h1></body></html>");
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\nConnection: close\r\n\r\n{}",
        response_body.len(),
        response_body
    );
    println!("--- taille de la réponse brute ---\n{} octets", response_body.len());

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("erreur d'écriture: {e}");
    }
}
