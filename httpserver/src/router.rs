use std::net::TcpStream;

use http::httprequest::HttpRequest;

use crate::handler::{Handler, StaticPageHandler, WebServiceHandler};

pub struct Router {}

impl Router {
    pub fn route(request: HttpRequest, stream: &mut TcpStream) {
        match request.method {
            http::httprequest::Method::Get => match &request.resource {
                http::httprequest::Resource::Path(path) => {
                    let route: Vec<&str> = path.split("/").collect();

                    match route[1] {
                        "api" => {
                            let response = WebServiceHandler::handle(&request);
                            response.send_response(stream).unwrap();
                        }

                        _ => {
                            let response = StaticPageHandler::handle(&request);
                            response.send_response(stream).unwrap();
                        }
                    }
                }
            },

            _ => {
                let response = WebServiceHandler::handle(&request);
                response.send_response(stream).unwrap();
            }
        }
    }
}
