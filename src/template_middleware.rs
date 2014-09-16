use std::io::{ MemReader, MemWriter };
use serialize::Encodable;

use iron::{ AroundMiddleware, Handler, Request, Response, IronResult };
use iron::status;
use iron::headers::response;
use iron::headers::content_type::MediaType;

use typemap::{ TypeMap, Assoc };
use http::headers::response::HeaderCollection;

use mustache;
use mustache::Context;
use mustache::encoder::{ Encoder, Error };

pub struct Data;

pub struct Template {
    // should be a cache of templates
    template: mustache::Template
}

pub struct TemplateHandler<H: Handler> {
    template: Template,
    handler: H
}

impl Assoc<'a, Encodable<Encoder<'a>, Error>> for Data {}

impl Template {
    pub fn new() -> Template {
        let context = Context::new(Path::new("views"));
        let template = context.compile_path(Path::new("index")).ok().expect("The template is not valid!");

        Template {
            template: template
        }
    }

    fn render_view<'a, D: Encodable<Encoder<'a>, Error>>(&self, name: String, data: D) -> MemReader {
        let mut mw = MemWriter::new();
        self.template.render(&mut mw, &data).ok().expect("The rendering of the template has failed!");

        MemReader::new(mw.unwrap())
    }
}

impl AroundMiddleware for Template {
    fn around<H>(self, handler: H) -> Box<Handler + Send + Sync> where H: Handler {
        box TemplateHandler {
            template: self,
            handler: handler
        } as Box<Handler + Send + Sync>
    }
}

impl<H: Handler> TemplateHandler<H> {
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
}

impl<H: Handler> Handler for TemplateHandler<H> {
    fn call(&self, req: &mut Request) -> IronResult<Response> {
        let res = self.handler.call(req);
        let name = res.body.read_to_string().ok().expect("The view's name is not valid!");
        let data = res.extenstions.find::<'a, Data, Encodable<Encoder<'a>, Error>>();
        let view = self.template.render_view(name, data);
        TemplateHandler<H>::html_response(view)
    }
}

