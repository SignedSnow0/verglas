use std::{collections::HashMap, sync::Arc};

pub trait RequestHandler {
    type Body: Into<String>;

    fn handle_request(&self) -> Option<Self::Body>;
}

pub struct Router<T> {
    get_endpoints: HashMap<String, Arc<dyn RequestHandler<Body = T>>>,
    post_endpoints: HashMap<String, Arc<dyn RequestHandler<Body = T>>>,
    put_endpoints: HashMap<String, Arc<dyn RequestHandler<Body = T>>>,
    delete_endpoints: HashMap<String, Arc<dyn RequestHandler<Body = T>>>,
}

impl<T> Router<T> {
    pub fn new() -> Router<T> {
        Router {
            get_endpoints: HashMap::new(),
            post_endpoints: HashMap::new(),
            put_endpoints: HashMap::new(),
            delete_endpoints: HashMap::new(),
        }
    }

    pub fn add_get_endpoint(
        &mut self,
        endpoint: &str,
        handler: impl RequestHandler<Body = T> + 'static,
    ) {
        if self.get_endpoints.contains_key(endpoint) {
            return;
        }

        self.get_endpoints
            .insert(endpoint.to_string(), Arc::new(handler));
    }

    pub fn get_get_endpoint(&self, endpoint: &str) -> Option<Arc<dyn RequestHandler<Body = T>>> {
        self.get_endpoints.get(endpoint).cloned()
    }
}
