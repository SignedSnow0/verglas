#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: String,
}

pub struct Response {
    pub status_code: u16,
    pub body: String,
}

// https://datatracker.ietf.org/doc/html/rfc2616#section-5.1.1
impl TryFrom<&str> for Method {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            _ => Err("Invalid method"),
        }
    }
}

// https://datatracker.ietf.org/doc/html/rfc2616#section-5
impl TryFrom<&str> for Request {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (method, uri) = {
            let request_line = value.lines().next().ok_or("Empty request")?;
            let mut tokens = request_line.split_whitespace();

            let method = tokens.next().ok_or("Empty request")?.try_into()?;

            let uri = tokens.next().ok_or("Empty request")?.to_string();

            (method, uri)
        };

        Ok(Self { method, uri })
    }
}

// https://datatracker.ietf.org/doc/html/rfc2616#section-6
impl Into<String> for Response {
    fn into(self) -> String {
        let status_code = match self.status_code {
            200 => "OK",
            404 => "Not Found",
            _ => "Internal Server Error",
        };

        let content_length = self.body.len();

        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code, status_code, content_length, self.body
        )
    }
}

#[cfg(test)]
mod test {
    use crate::http::*;

    #[test]
    fn parse_method() {
        let method: Method = "GET".try_into().unwrap();
        assert_eq!(method, Method::GET);

        let method: Method = "POST".try_into().unwrap();
        assert_eq!(method, Method::POST);

        let method: Method = "PUT".try_into().unwrap();
        assert_eq!(method, Method::PUT);

        let method: Method = "DELETE".try_into().unwrap();
        assert_eq!(method, Method::DELETE);

        let method: Result<Method, _> = "INVALID".try_into();
        assert_eq!(method, Err("Invalid method"));
    }

    #[test]
    fn parse_request() {
        let request: Request = "GET / HTTP/1.1".try_into().unwrap();

        assert_eq!(request.method, Method::GET);
        assert_eq!(request.uri, "/".to_string());

        let request: Request = "POST /login HTTP/1.1".try_into().unwrap();

        assert_eq!(request.method, Method::POST);
        assert_eq!(request.uri, "/login".to_string());

        let request: Result<Request, _> = "".try_into();
        assert!(request.is_err());

        let request: Result<Request, _> = "POSt / HTTP/1.2".try_into();
        assert!(request.is_err());
    }
}
