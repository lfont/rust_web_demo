use iron::{ Request, Response, IronResult };
use router::Router;

use mustache::MapBuilder;

use template_middleware::view_response;

fn root_handler(_: &mut Request) -> IronResult<Response> {
    let data = MapBuilder::new()
        .insert_str("message", "hello")
        .build();

    Ok(view_response("index".to_string(), data))
}

pub fn route() -> Router {
    let mut router = Router::new();
    router.get("/", root_handler);
    router
}

