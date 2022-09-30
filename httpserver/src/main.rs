use httpserver::server::Server;

fn main() {
    let server = Server::new("0.0.0.0:3000");

    server.run();
}
