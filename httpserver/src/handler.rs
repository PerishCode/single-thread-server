use std::{collections::HashMap, env, fs};

use http::{
    httprequest::{HttpRequest, Resource},
    httpresponse::HttpResponse,
};
use serde::{Deserialize, Serialize};
pub trait Handler {
    fn handle(request: &HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_data: String,
    order_status: String,
}

pub struct WebServiceHandler {}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);

        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_contents = fs::read_to_string(full_path);

        let orders: Vec<OrderStatus> =
            serde_json::from_str(json_contents.unwrap().as_str()).unwrap();

        orders
    }
}

impl Handler for WebServiceHandler {
    // fn handle(request: &HttpRequest) -> HttpResponse {

    //     let Resource::Path(s) = &request.resource;
    //     let route: Vec<&str> = s.split("/")

    //     HttpResponse::new("200", None, None)

    // }
    fn handle(_request: &HttpRequest) -> HttpResponse {
        HttpResponse::new("200", None, None)
    }
}

pub struct StaticPageHandler {}

impl Handler for StaticPageHandler {
    fn handle(request: &HttpRequest) -> HttpResponse {
        let Resource::Path(s) = &request.resource;
        let route: Vec<&str> = s.split("/").collect();

        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "headlth" => HttpResponse::new("200", None, Self::load_file("health.html")),
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut headers: HashMap<&str, &str> = HashMap::new();

                    if path.ends_with(".css") {
                        headers.insert("Content-type", "text/css");
                    } else if path.ends_with(".js") {
                        headers.insert("Content-Type", "text/javascript");
                    } else {
                        headers.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(headers), Some(contents))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            },
        }
    }
}

pub struct PageNotFoundHandler {}

impl Handler for PageNotFoundHandler {
    fn handle(_request: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}
