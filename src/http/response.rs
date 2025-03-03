use super::Response;

// https://datatracker.ietf.org/doc/html/rfc2616#section-6
impl Response {
    pub fn empty() -> Self {
        Self {
            status_code: 200,
            body: "".to_string(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            status_code: 404,
            body: "".to_string(),
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            status_code: 500,
            body: "".to_string(),
        }
    }
}

impl From<Response> for String {
    fn from(value: Response) -> Self {
        let status_code = match value.status_code {
            200 => "OK",
            404 => "Not Found",
            _ => "Internal Server Error",
        };

        let content_length = value.body.len();

        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            value.status_code, status_code, content_length, value.body
        )
    }
}
