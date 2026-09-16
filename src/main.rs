use std::net::TcpListener;

mod http;
mod connection;

mod router;
use connection::handle_connection;
use router::logique_routeur::build_router;

fn main() -> std::io::Result<()> {
    // charge le .env s'il existe (silencieux si absent) : peuple les vraies
    // variables d'environnement du processus, std::env::var peut alors les lire.
    dotenvy::dotenv().ok();

    // DEBUG=false coupe tout log, quoi que dise RUST_LOG — l'interrupteur maître.
    // DEBUG absent ou =true : comportement normal (RUST_LOG prioritaire, sinon "debug").
    let debug_enabled = std::env::var("DEBUG")
        .map(|v| !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true);

    let filter = if debug_enabled {
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug"))
    } else {
        tracing_subscriber::EnvFilter::new("off")
    };

    // installe le subscriber par défaut : sans lui, les macros tracing (info!, error!...)
    // n'affichent rien nulle part, elles ne font qu'émettre des évènements dans le vide.
    tracing_subscriber::fmt().with_env_filter(filter).init();
    let style_html = std::fs::read_to_string("static/main.css")?;
    // construite une seule fois au démarrage — jamais par requête, cf. le
    // commentaire de build_router() dans router/logique_routeur.rs.
    let router = build_router();

    let listener = TcpListener::bind("127.0.0.1:3000")?;
    // flag lu une seule fois au démarrage : active le détail du chronométrage par étape.
    // usage : SERVEUR_PROFILE=1 cargo run --release
    let profiling = std::env::var("SERVEUR_PROFILE").is_ok();
    tracing::info!("écoute sur http://127.0.0.1:3000");

    for stream in listener.incoming() {
        let stream = stream?;
        // une seule connexion à la fois pour l'instant : pas de thread, pas d'async.
        // objectif : voir ce qui transite sur le socket avant d'introduire de la concurrence.
        handle_connection(stream, &style_html, profiling, &router);
    }

    Ok(())
}


