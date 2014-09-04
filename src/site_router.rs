use iron::{ Request, Response, IronResult };
use iron::status;
use router::Router;

pub fn route() -> Router {
    let mut router = Router::new();
    router.get("/", root_handler);
    router
}

fn root_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok, "hello"))
}
