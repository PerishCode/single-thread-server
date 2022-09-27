use std::{collections::HashMap, io::Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();

        if status_code != "200" {
            response.status_code = status_code;
        }

        response.headers = match &headers {
            Some(_) => headers,
            None => {
                let mut headers = HashMap::new();
                headers.insert("Content-Type", "text/html");
                Some(headers)
            }
        };

        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Not Defined",
        };

        response.body = body;

        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<(), ()> {
        let response = self.clone();
        let response_text = String::from(response);

        let _ = write!(write_stream, "{}", response_text);

        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = "".into();

        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }

        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(body) => body.as_str(),
            None => "",
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(response: HttpResponse<'a>) -> Self {
        let clone = response.clone();

        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &clone.version(),
            &clone.status_code(),
            &clone.status_text(),
            &clone.headers(),
            &response.body.unwrap().len(),
            &clone.body()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200", None, Some("xxxx".into()));

        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type", "text/html");
                Some(headers)
            },
            body: Some("xxxx".into()),
        };

        assert_eq!(response_actual, response_expected)
    }

    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new("404", None, Some("xxxx".into()));

        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type", "text/html");
                Some(headers)
            },
            body: Some("xxxx".into()),
        };

        assert_eq!(response_actual, response_expected)
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type", "text/html");
                Some(headers)
            },
            body: Some("xxxx".into()),
        };

        let http_text: String = response_expected.into();

        let actual_string: String =
            "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 4\r\n\r\nxxxx"
                .into();

        assert_eq!(http_text, actual_string)
    }
}
