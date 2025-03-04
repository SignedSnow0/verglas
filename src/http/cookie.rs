#[derive(Debug)]
pub struct Cookie {
    pub(super) key: String,
    pub(super) value: String,
    pub(super) secure: bool,
    pub(super) http_only: bool,
    pub(super) domain: Option<String>,
    pub(super) path: Option<String>,
    pub(super) expires: Option<String>,
    pub(super) max_age: Option<String>,
    pub(super) same_site: Option<String>,
}

#[derive(Default, Clone)]
pub struct NoKey;
#[derive(Default, Clone)]
pub struct Key(String);
#[derive(Default, Clone)]
pub struct NoValue;
#[derive(Default, Clone)]
pub struct Value(String);

#[derive(Default)]
pub struct CookieBuilder<Tkey, Tvalue> {
    key: Tkey,
    value: Tvalue,
    secure: bool,
    http_only: bool,
    domain: Option<String>,
    path: Option<String>,
    expires: Option<String>,
    max_age: Option<String>,
    same_site: Option<String>,
}

impl CookieBuilder<NoKey, NoValue> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Tvalue> CookieBuilder<NoKey, Tvalue> {
    pub fn with_key(self, key: &str) -> CookieBuilder<Key, Tvalue> {
        CookieBuilder {
            key: Key(key.to_string()),
            value: self.value,
            secure: self.secure,
            http_only: self.http_only,
            domain: self.domain,
            path: self.path,
            expires: self.expires,
            max_age: self.max_age,
            same_site: self.same_site,
        }
    }
}

impl<Tkey> CookieBuilder<Tkey, NoValue> {
    pub fn with_value(self, value: &str) -> CookieBuilder<Tkey, Value> {
        CookieBuilder {
            key: self.key,
            value: Value(value.to_string()),
            secure: self.secure,
            http_only: self.http_only,
            domain: self.domain,
            path: self.path,
            expires: self.expires,
            max_age: self.max_age,
            same_site: self.same_site,
        }
    }
}

impl CookieBuilder<Key, Value> {
    pub fn build(self) -> Cookie {
        Cookie {
            key: self.key.0,
            value: self.value.0,
            secure: self.secure,
            http_only: self.http_only,
            domain: self.domain,
            path: self.path,
            expires: self.expires,
            max_age: self.max_age,
            same_site: self.same_site,
        }
    }
}

impl<Tkey, Tvalue> CookieBuilder<Tkey, Tvalue> {
    pub fn with_secure(self, secure: bool) -> Self {
        Self { secure, ..self }
    }

    pub fn with_http_only(self, http_only: bool) -> Self {
        Self { http_only, ..self }
    }

    pub fn with_domain(self, domain: &str) -> Self {
        Self {
            domain: Some(domain.to_string()),
            ..self
        }
    }

    pub fn with_path(self, path: &str) -> Self {
        Self {
            path: Some(path.to_string()),
            ..self
        }
    }

    pub fn with_expires(self, expires: &str) -> Self {
        Self {
            expires: Some(expires.to_string()),
            ..self
        }
    }

    pub fn with_max_age(self, max_age: &str) -> Self {
        Self {
            max_age: Some(max_age.to_string()),
            ..self
        }
    }

    pub fn with_same_site(self, same_site: &str) -> Self {
        Self {
            same_site: Some(same_site.to_string()),
            ..self
        }
    }
}

#[derive(Debug)]
pub struct RequestCookie {
    pub key: String,
    pub value: String,
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
    use super::CookieBuilder;

    #[test]
    fn test_empty() {
        let cookie = CookieBuilder::new()
            .with_key("testkey")
            .with_value("testvalue")
            .build();

        assert_eq!(String::from(&cookie), "Set-Cookie: testkey=testvalue");
    }
}
