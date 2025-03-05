use super::cookie::Cookie;

// https://datatracker.ietf.org/doc/html/rfc2616#section-6
#[derive(Debug)]
pub struct Response {
    status_code: u16,
    body: Option<String>,
    cookies: Vec<Cookie>,
}

#[derive(Default, Clone)]
pub struct NoStatusCode;
#[derive(Default, Clone)]
pub struct StatusCode(u16);

#[derive(Default)]
pub struct ResponseBuilder<TStatusCode> {
    status_code: TStatusCode,
    body: Option<String>,
    cookies: Vec<Cookie>,
}

impl ResponseBuilder<NoStatusCode> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_status_code(self, status_code: u16) -> ResponseBuilder<StatusCode> {
        ResponseBuilder {
            status_code: StatusCode(status_code),
            body: self.body,
            cookies: self.cookies,
        }
    }

    pub fn empty(self) -> ResponseBuilder<StatusCode> {
        ResponseBuilder {
            status_code: StatusCode(200),
            body: None,
            cookies: self.cookies,
        }
    }

    pub fn not_found(self) -> ResponseBuilder<StatusCode> {
        ResponseBuilder {
            status_code: StatusCode(404),
            body: None,
            cookies: self.cookies,
        }
    }

    pub fn internal_server_error(self) -> ResponseBuilder<StatusCode> {
        ResponseBuilder {
            status_code: StatusCode(500),
            body: self.body,
            cookies: self.cookies,
        }
    }
}

impl<TStatusCode> ResponseBuilder<TStatusCode> {
    pub fn with_body(self, body: &str) -> ResponseBuilder<TStatusCode> {
        ResponseBuilder {
            status_code: self.status_code,
            body: Some(body.to_string()),
            cookies: self.cookies,
        }
    }

    pub fn with_cookies(self, cookies: Vec<Cookie>) -> ResponseBuilder<TStatusCode> {
        ResponseBuilder {
            status_code: self.status_code,
            body: self.body,
            cookies,
        }
    }
}

impl ResponseBuilder<StatusCode> {
    pub fn build(self) -> Response {
        Response {
            status_code: self.status_code.0,
            body: self.body,
            cookies: self.cookies,
        }
    }
}

impl From<&Response> for String {
    fn from(value: &Response) -> Self {
        let status_code = match value.status_code {
            200 => "OK",
            404 => "Not Found",
            _ => "Internal Server Error",
        };

        let content_length = match &value.body {
            Some(body) => body.len(),
            None => 0,
        };

        let response = format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n",
            value.status_code, status_code, content_length
        );

        let response = value.cookies.iter().fold(response, |acc, cookie| {
            format!("{}{}", acc, String::from(cookie))
        });

        match &value.body {
            Some(body) => format!("{}\r\n{}", response, body),
            None => format!("{}\r\n", response),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::http::cookie::CookieBuilder;

    use super::*;

    #[test]
    fn test_empty_responses() {
        let response = ResponseBuilder::new().empty().build();
        assert_eq!(
            String::from(&response),
            "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n"
        );

        let response = ResponseBuilder::new().not_found().build();
        assert_eq!(
            String::from(&response),
            "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n"
        );

        let response = ResponseBuilder::new().internal_server_error().build();
        assert_eq!(
            String::from(&response),
            "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n"
        );
    }

    #[test]
    fn test_cookies() {
        let cookie = CookieBuilder::new()
            .with_key("key")
            .with_value("value")
            .build();

        let response = ResponseBuilder::new()
            .empty()
            .with_cookies(vec![cookie])
            .build();

        let expected = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\nSet-Cookie: key=value\r\n\r\n";

        assert_eq!(String::from(&response), expected);
    }
}
