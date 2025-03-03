use super::Request;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

impl Request {
    pub fn contains_header(&self, header: &str) -> bool {
        self.headers.contains_key(header)
    }

    pub fn get_header(&self, header: &str) -> Option<&String> {
        self.headers.get(header)
    }
}

impl TryFrom<&mut TcpStream> for Request {
    type Error = &'static str;

    fn try_from(value: &mut TcpStream) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);

        // Raw header parsing from TcpStream
        let header = {
            let mut header = Vec::new();

            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                if line == "\r\n" {
                    break;
                }
                header.push(line.trim_end().to_owned());
            }

            header
        };

        // Parsing of request line
        let (method, uri, version) = {
            let request_line = header.get(0).ok_or("Empty request")?;
            let mut tokens = request_line.split_whitespace();

            let method = tokens.next().ok_or("No method found")?.try_into()?;

            let uri = tokens.next().ok_or("No uri found")?.try_into()?;

            let version = tokens.next().ok_or("No http version found")?.try_into()?;

            (method, uri, version)
        };

        let headers: HashMap<String, String> = header
            .into_iter()
            .skip(1)
            .map_while(|line| {
                let mut tokens = line.splitn(2, ':');

                let key = if let Some(key) = tokens.next() {
                    key
                } else {
                    return None;
                };

                let value = if let Some(value) = tokens.next() {
                    value.trim()
                } else {
                    return None;
                };

                Some((key.to_owned(), value.to_owned()))
            })
            .collect();

        match headers.get("Content-Lenght") {
            Some(lenght) => {
                let Ok(length) = lenght.parse::<u32>() else {
                    return Err("Error parsing content lenght");
                };

                if length == 0 {
                    return Ok(Self {
                        method,
                        uri,
                        version,
                        headers,
                        body: None,
                    });
                }

                let mut body = vec![0; length as usize];
                reader.read_exact(&mut body).unwrap();

                let body = String::from_utf8(body).unwrap();

                Ok(Self {
                    method,
                    uri,
                    version,
                    headers,
                    body: Some(body),
                })
            }
            None => Ok(Self {
                method,
                uri,
                version,
                headers,
                body: None,
            }),
        }
    }
}

impl TryFrom<&str> for Request {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let header = value
            .lines()
            .take_while(|line| !line.is_empty())
            .collect::<Vec<_>>();

        let (method, uri, version) = {
            let request_line = header.get(0).ok_or("Empty request")?;
            let mut tokens = request_line.split_whitespace();

            let method = tokens.next().ok_or("Empty request")?.try_into()?;

            let uri = tokens.next().ok_or("Empty request")?.try_into()?;

            let version = tokens.next().ok_or("Empty request")?.try_into()?;

            (method, uri, version)
        };

        let headers = header
            .into_iter()
            .skip(1)
            .map_while(|line| {
                let mut tokens = line.splitn(2, ':');

                let key = if let Some(key) = tokens.next() {
                    key
                } else {
                    return None;
                };

                let value = if let Some(value) = tokens.next() {
                    value.trim()
                } else {
                    return None;
                };

                Some((key.to_owned(), value.to_owned()))
            })
            .collect::<HashMap<_, _>>();

        match headers.get("Content-Length") {
            Some(content_length) => {
                let content_length = content_length.parse::<u32>().unwrap();
                if content_length == 0 {
                    return Ok(Self {
                        method,
                        uri,
                        version,
                        headers,
                        body: None,
                    });
                }

                let body_start = value.len() - content_length as usize;
                let body = value.as_bytes()[body_start..].to_vec();

                Ok(Self {
                    method,
                    uri,
                    version,
                    headers,
                    body: Some(String::from_utf8(body).unwrap()),
                })
            }
            None => {
                return {
                    Ok(Self {
                        method,
                        uri,
                        version,
                        headers,
                        body: None,
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::http::{Method, Request};

    #[test]
    fn parse_request() {
        let request = Request::try_from("GET / HTTP/1.1\r\n\r\n").unwrap();

        assert_eq!(request.method, Method::Get);
        assert_eq!(request.uri.path, "/".to_string());
        assert_eq!(request.uri.attributes, vec![]);

        let request = Request::try_from("POST /login?name=foo&password=bar HTTP/1.1").unwrap();

        assert_eq!(request.method, Method::Post);
        assert_eq!(request.uri.path, "/login".to_string());
        assert_eq!(
            request.uri.attributes,
            vec![
                ("name".to_string(), "foo".to_string()),
                ("password".to_string(), "bar".to_string())
            ]
        );

        let request: Result<Request, _> = "".try_into();
        assert!(request.is_err());

        let request: Result<Request, _> = "POSt / HTTP/1.2".try_into();
        assert!(request.is_err());
    }
}
