use serialize::Encodable;

use iron::{ Request, Response, IronResult };
use iron::status;
use router::Router;

use template_middleware::ViewData;

#[deriving(Encodable)]
struct Index<'a> {
    message: &'a str
}

fn root_handler(req: &mut Request) -> IronResult<Response> {
    let res = Response::with(status::Ok, ViewData {
        name: "index",
        data: Index { message: "hello" }
    });

    Ok(res)
}

pub fn route() -> Router {
    let mut router = Router::new();
    router.get("/", root_handler);
    router
}

