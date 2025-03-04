use super::{Cookie, Response};

// https://datatracker.ietf.org/doc/html/rfc2616#section-6
impl Response {
    pub fn empty() -> Self {
        Self {
            status_code: 200,
            body: "".to_string(),
            cookies: Default::default(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            status_code: 404,
            body: "".to_string(),
            cookies: Default::default(),
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            status_code: 500,
            body: "".to_string(),
            cookies: Default::default(),
        }
    }

    pub fn add_cookie(&mut self, cookie: Cookie) {
        if self.cookies.iter().any(|c| c.key == cookie.key) {
            return;
        }

        self.cookies.push(cookie);
    }
}

impl From<&Response> for String {
    fn from(value: &Response) -> Self {
        let status_code = match value.status_code {
            200 => "OK",
            404 => "Not Found",
            _ => "Internal Server Error",
        };

        let content_length = value.body.len();

        let response = format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n",
            value.status_code, status_code, content_length
        );

        let response = value.cookies.iter().fold(response, |acc, cookie| {
            format!("{}{}", acc, String::from(cookie))
        });

        if value.body.is_empty() {
            format!("{}\r\n", response)
        } else {
            format!("{}\r\n{}", response, value.body)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_responses() {
        let response = Response::empty();
        let expected = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
        assert_eq!(String::from(&response), expected);

        let response = Response::not_found();
        let expected = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
        assert_eq!(String::from(&response), expected);

        let response = Response::internal_server_error();
        let expected = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n";
        assert_eq!(String::from(&response), expected);
    }

    #[test]
    fn test_cookies() {
        let cookie = Cookie::new("key", "value");

        let mut response = Response::empty();
        response.add_cookie(cookie);

        let expected = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\nSet-Cookie: key=value\r\n\r\n";

        assert_eq!(String::from(&response), expected);
    }
}
