use super::{Method, Uri, Version};

// https://datatracker.ietf.org/doc/html/rfc2616#section-5.1.1
impl TryFrom<&str> for Method {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Method::Get),
            "HEAD" => Ok(Method::Head),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            "CONNECT" => Ok(Method::Connect),
            "OPTIONS" => Ok(Method::Options),
            "TRACE" => Ok(Method::Trace),
            _ => Err("Invalid method"),
        }
    }
}

impl TryFrom<&str> for Version {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "HTTP/1.0" => Ok(Version::Http1_0),
            "HTTP/1.1" => Ok(Version::Http1_1),
            _ => Err("Invalid version"),
        }
    }
}

impl TryFrom<&str> for Uri {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tokens = value.splitn(2, '?');

        let path = tokens.next().ok_or("Empty URI")?.to_string();

        let attributes = match tokens.next() {
            Some(query) => query
                .split('&')
                .map_while(|pair| {
                    let mut tokens = pair.splitn(2, '=');

                    let key = if let Some(key) = tokens.next() {
                        key
                    } else {
                        return None;
                    };

                    let value = if let Some(value) = tokens.next() {
                        value
                    } else {
                        return None;
                    };

                    Some((key.to_owned(), value.to_owned()))
                })
                .collect(),
            None => vec![],
        };

        Ok(Self { path, attributes })
    }
}

#[cfg(test)]
mod test {
    use crate::http::Method;

    #[test]
    fn parse_method() {
        let method: Method = "GET".try_into().unwrap();
        assert_eq!(method, Method::Get);

        let method: Method = "HEAD".try_into().unwrap();
        assert_eq!(method, Method::Head);

        let method: Method = "POST".try_into().unwrap();
        assert_eq!(method, Method::Post);

        let method: Method = "PUT".try_into().unwrap();
        assert_eq!(method, Method::Put);

        let method: Method = "DELETE".try_into().unwrap();
        assert_eq!(method, Method::Delete);

        let method: Method = "CONNECT".try_into().unwrap();
        assert_eq!(method, Method::Connect);

        let method: Method = "OPTIONS".try_into().unwrap();
        assert_eq!(method, Method::Options);

        let method: Method = "TRACE".try_into().unwrap();
        assert_eq!(method, Method::Trace);

        let method: Result<Method, _> = "INVALID".try_into();
        assert_eq!(method, Err("Invalid method"));
    }
}
