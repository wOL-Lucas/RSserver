mod routes;
use routes::Router;

fn main() {
    let router = Router::new();
    router.init_server();
}
