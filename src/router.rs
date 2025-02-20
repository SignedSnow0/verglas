use crate::http::Method;

pub struct Route {
    pub uri: String,
    pub method: Method,
    pub handler: fn() -> String,
}

pub struct Router {
    routes: Vec<Route>,
}

pub struct RouterBuilder {
    routes: Vec<Route>,
}

impl RouterBuilder {
    pub fn new() -> Self {
        RouterBuilder { routes: Vec::new() }
    }

    pub fn with_route(mut self, route: Route) -> Self {
        self.routes.push(route);
        self
    }

    pub fn build(self) -> Router {
        Router {
            routes: self.routes,
        }
    }
}

impl Router {
    pub fn dispatch(&self, uri: &str, method: Method) -> String {
        for route in &self.routes {
            if route.uri == uri && route.method == method {
                return (route.handler)();
            }
        }

        String::new()
    }
}
