use crate::http::{
    request::Request,
    response::{Response, ResponseBuilder},
    Method,
};

pub struct Route {
    pub path: String,
    pub method: Method,
    pub handler: fn(&Request) -> Response,
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
    pub fn dispatch(&self, request: &Request) -> Response {
        for route in &self.routes {
            if route.path == request.uri.path && route.method == request.method {
                return (route.handler)(request);
            }
        }

        ResponseBuilder::new().not_found().build()
    }
}
