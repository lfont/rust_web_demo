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
    use iron::{ ChainBuilder, Chain };
    use mount::Mount;

    use template_middleware;
    use site_router;
    use api_router;

    pub fn mount() -> Mount {
        let mut chain = ChainBuilder::new(site_router::route());
        chain.link_after(template_middleware::Template::new());

        let mut mount = Mount::new();
        mount.mount("/", chain);
        mount.mount("/api", api_router::route());
        mount
    }
}
