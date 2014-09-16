use serialize::Encodable;

use iron::{ Request, Response, IronResult };
use iron::status;
use router::Router;

use mustache::encoder::{ Encoder, Error };

use template_middleware::Data;

#[deriving(Encodable)]
struct Index<'a> {
    message: &'a str
}

fn root_handler(req: &mut Request) -> IronResult<Response> {
    let res = Response::with(status::Ok, "index");
    res.extensions.insert::<'a, Data, Encodable<Encoder<'a>, Error>>(Index { message: "hello" });
    Ok(res)
}

pub fn route() -> Router {
    let mut router = Router::new();
    router.get("/", root_handler);
    router
}

