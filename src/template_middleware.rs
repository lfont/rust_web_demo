use std::io::{ MemReader, MemWriter };
use std::path::BytesContainer;

use iron::{ AroundMiddleware, Handler, Request, Response, IronResult };
use iron::status;
use iron::headers::response;
use iron::headers::content_type::MediaType;

use typemap::{ Assoc, TypeMap };
use http::headers::response::HeaderCollection;

use mustache;
use mustache::Context;
use mustache::Data;

pub struct Template {
    // should be a cache of templates
    template: mustache::Template
}

pub struct TemplateHandler<H: Handler> {
    template: Template,
    handler: H
}

impl Template {
    pub fn new() -> Template {
        let context = Context::new(Path::new("views"));
        let template = context.compile_path(Path::new("index")).ok().expect("The template is not valid!");

        Template {
            template: template
        }
    }

    fn render_view<'a>(&self, _: String, data: &Data<'a>) -> MemReader {
        let mut mw = MemWriter::new();
        self.template.render_data(&mut mw, data);

        MemReader::new(mw.unwrap())
    }
}

impl<'a> Assoc<Data<'a>> for Template {}

impl AroundMiddleware for Template {
    fn around<H>(self, handler: H) -> Box<Handler + Send + Sync> where H: Handler {
        box TemplateHandler {
            template: self,
            handler: handler
        } as Box<Handler + Send + Sync>
    }
}

pub fn view_response(name: String, data: Data<'static>) -> Response {
    let reader = MemReader::new(name.container_into_owned_bytes());

    let mut extensions = TypeMap::new();
    extensions.insert::<Template, Data>(data);

    Response {
        headers: HeaderCollection::new(),
        status: Some(status::Ok),
        body: Some(box reader as Box<Reader + Send>),
        extensions: extensions
    }
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

impl<H: Handler> Handler for TemplateHandler<H> {
    fn call(&self, req: &mut Request) -> IronResult<Response> {
        let res = self.handler.call(req).ok().expect("Cannot get the response!");
        let name = res.body.expect("The body is not valid!").read_to_string().ok().expect("The view's name is not valid!");
        let data = res.extensions.find::<Template, Data>().unwrap();
        let view = self.template.render_view(name, data);
        Ok(html_response(view))
    }
}

