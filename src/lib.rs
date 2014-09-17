extern crate serialize;
extern crate http;
extern crate typemap;
extern crate iron;
extern crate mount;
extern crate router;
extern crate mustache;

mod template_middleware;
mod site_router;
mod api_router;

pub mod app {
    use iron::AroundMiddleware;
    use mount::Mount;

    use template_middleware;
    use site_router;
    use api_router;

    pub fn mount() -> Mount {
        let mut mount = Mount::new();
        mount.mount("/", template_middleware::Template::new().around(site_router::route()));
        mount.mount("/api", api_router::route());
        mount
    }
}
