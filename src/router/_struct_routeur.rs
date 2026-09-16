use crate::http::_struct::{Method, Request, Response};

// Données partagées : tout ce qu'un handler peut avoir besoin de lire
// mais qui ne fait PAS partie de la requête (contenu statique déjà chargé,
// futur pool de connexion DB, config...). Sans ça, un handler n'a accès
// qu'à &Request — aucun moyen de lui passer autre chose. C'est l'équivalent
// simplifié de l'Arc<RuniqueEngine> passé à chaque vue dans Runique.
pub struct Data<'a> {
    pub style_html: &'a str,
}

pub struct Route {
    pub method: Method,
    // &'static str, pas String : les chemins de route sont des littéraux connus
    // à la compilation ("/", "/about") — aucune raison de les allouer sur le tas.
    pub path: &'static str,
    // fn(&Request, &Data) -> Response : même principe d'élision de lifetime
    // que precedemment pour Request — Rust traite chaque lifetime non-nommée
    // d'un TYPE fn pointeur comme universelle (for<'a>, for<'b>...), donc ce
    // handler fonctionne pour n'importe quelle Request<'a> ET n'importe quel
    // Data<'b>, indépendamment l'un de l'autre.
    pub handler: fn(&Request, &Data) -> Response,
}

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new(routes: Vec<Route>) -> Self {
        Router { routes }
    }

    // Cherche la première route dont (method, path) correspond à la requête,
    // et appelle son handler avec l'état applicatif. None si aucune route ne
    // correspond — c'est à l'appelant (connection.rs) de décider quoi faire
    // du cas "pas trouvé" (répondre 404 via not_found), pas au routeur lui-même.
    pub fn dispatch(&self, request: &Request, state: &Data) -> Option<Response> {
        for route in &self.routes {
            if route.method == request.method && route.path == request.path {
                return Some((route.handler)(request, state));
            }
        }
        None
    }
}
