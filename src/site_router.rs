use std::io::{ MemReader, MemWriter };
use serialize::Encodable;

use typemap::TypeMap;
use http::headers::response::HeaderCollection;

use iron::{ Request, Response, IronResult };
use iron::status;
use iron::headers::response;
use iron::headers::content_type::MediaType;
use router::Router;

use mustache::Context;
use mustache::encoder::{ Encoder, Error };

#[deriving(Encodable)]
struct Index<'a> {
    message: &'a str
}

fn render_view<'a, T: Encodable<Encoder<'a>, Error>>(view: &str, data: &T) -> MemReader {
    let context = Context::new(Path::new("views"));
    let template = context.compile_path(Path::new(view)).ok().expect("The template is not valid!");

    let mut mw = MemWriter::new();
    template.render(&mut mw, data).ok().expect("The rendering of the template has failed!");

    MemReader::new(mw.unwrap())
}

fn html_response(view: MemReader) -> Response {
    let mut headers = HeaderCollection::new();
    headers.insert(response::ContentType(MediaType::new("text".to_string(), "html".to_string(), vec![])));

    Response {
        headers: headers,
        status: Some(status::Ok),
        body: Some(box view as Box<Reader + Send>),
        extensions: TypeMap::new()
    }
}

fn root_handler(req: &mut Request) -> IronResult<Response> {
    let index = Index { message: "hello" };
    let view = render_view("index", &index);
    Ok(html_response(view))
}

pub fn route() -> Router {
    let mut router = Router::new();
    router.get("/", root_handler);
    router
}

