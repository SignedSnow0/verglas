type Url = String;

#[derive(Debug)]
pub enum Method {
    Get(Url),
    Post(Url),
    Put(Url),
    Delete(Url),
    Head,
    Options,
    Trace,
}

impl TryFrom<&str> for Method {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tokens = value.split(" ");

        let method = match tokens.next() {
            Some(method) => method,
            None => return Err("Error parsing request method!"),
        };

        match method {
            "GET" => {
                let url = match tokens.next() {
                    Some(url) => url,
                    None => return Err("Error parsing request url!"),
                };

                Ok(Method::Get(url.to_owned()))
            }
            "POST" => {
                let url = match tokens.next() {
                    Some(url) => url,
                    None => return Err("Error parsing request url!"),
                };

                Ok(Method::Get(url.to_owned()))
            }
            _ => Err("Unsupported request method!"),
        }
    }
}
