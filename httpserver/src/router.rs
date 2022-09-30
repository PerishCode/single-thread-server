use std::net::TcpStream;

use http::httprequest::HttpRequest;

pub struct Router {}

impl Router {
    pub fn route(_request: HttpRequest, _stream: &mut TcpStream) {}
}
