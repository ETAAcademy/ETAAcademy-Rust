use crate::handler::{WebServiceHandler, StaticPageHandler};
use super::handler::{Handler, PageNotFoundHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;
impl Router{
    pub fn route(req: HttpRequest, stream:&mut impl Write) -> (){
        match req.method {
            httprequest::Method::GET => match  &req.resource{
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        //localhost:3000/api
                        "api" => {
                            let resp : HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _=>{
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp:HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
       
    }
}