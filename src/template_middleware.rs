use std::io::{ MemReader, MemWriter };
use serialize::Encodable;

use iron::{ AfterMiddleware, Request, Response, IronResult };
use iron::status;
use iron::headers::response;
use iron::headers::content_type::MediaType;

use typemap::TypeMap;
use http::headers::response::HeaderCollection;

use mustache;
use mustache::Context;
use mustache::encoder::{ Encoder, Error };

pub struct ViewData<'a> {
    name: &'a str,
    data: Encodable<Encoder<'a>, Error>+'a
}

pub struct Template {
    // should be a cache of templates
    template: mustache::Template
}

impl Template {
    pub fn new() -> Template {
        let context = Context::new(Path::new("views"));
        let template = context.compile_path(Path::new("index")).ok().expect("The template is not valid!");

        Template {
            template: template
        }
    }

    fn render_view(&self, view_data: ViewData) -> MemReader {
        let mut mw = MemWriter::new();
        self.template.render(&mut mw, &view_data.data).ok().expect("The rendering of the template has failed!");

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
}

impl AfterMiddleware for Template {
    fn after(&self, req: &mut Request, res: &mut Response) -> IronResult<Response> {
        let view = self.render_view(res.body);
        Ok(self.html_response(view))
    }
}
