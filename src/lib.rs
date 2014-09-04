extern crate iron;
extern crate mount;
extern crate router;

mod site_router;
mod api_router;

pub mod app {
    use mount::Mount;
    use site_router;
    use api_router;

    pub fn mount() -> Mount {
        let mut mount = Mount::new();
        mount.mount("/", site_router::route());
        mount.mount("/api", api_router::route());
        mount
    }
}
