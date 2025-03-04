use super::Cookie;

impl Cookie {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
            secure: false,
            http_only: false,
            domain: None,
            path: None,
            expires: None,
            max_age: None,
            same_site: None,
        }
    }
}

impl From<&Cookie> for String {
    fn from(value: &Cookie) -> Self {
        let cookie_str = "Set-Cookie: ".to_string();
        let cookie_str = format!("{} {}={}", cookie_str, value.key, value.value);

        let cookie_str = match &value.domain {
            Some(domain) => {
                let cookie_str = format!("{}; Domain={}", cookie_str, domain);
                cookie_str
            }
            None => cookie_str,
        };

        let cookie_str = match &value.path {
            Some(path) => {
                let cookie_str = format!("{}; Path={}", cookie_str, path);
                cookie_str
            }
            None => cookie_str,
        };

        let cookie_str = match &value.expires {
            Some(expires) => {
                let cookie_str = format!("{}; Expires={}", cookie_str, expires);
                cookie_str
            }
            None => cookie_str,
        };

        let cookie_str = match &value.max_age {
            Some(max_age) => {
                let cookie_str = format!("{}; Max-Age={}", cookie_str, max_age);
                cookie_str
            }
            None => cookie_str,
        };

        let cookie_str = match &value.same_site {
            Some(same_site) => {
                let cookie_str = format!("{}; SameSite={}", cookie_str, same_site);
                cookie_str
            }
            None => cookie_str,
        };

        let cookie_str = if value.secure {
            format!("{}; Secure", cookie_str)
        } else {
            cookie_str
        };

        let cookie_str = if value.http_only {
            format!("{}; HttpOnly", cookie_str)
        } else {
            cookie_str
        };

        format!("{}\r\n", cookie_str)
    }
}

#[cfg(test)]
mod test {
    use crate::http::Cookie;

    #[test]
    fn test_empty() {
        let cookie = Cookie::new("testkey", "testvalue");

        assert_eq!(String::from(&cookie), "Set-Cookie: testkey=testvalue");
    }
}
