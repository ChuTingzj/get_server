mod server;
use server::Server;
fn main() {
    let server = Server::new(7878);
    let listener = server.start().unwrap();
    Server::handle_stream(&listener)
}
