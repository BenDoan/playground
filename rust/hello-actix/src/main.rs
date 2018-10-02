extern crate actix;
extern crate actix_web;
extern crate env_logger;

#[macro_use] extern crate serde_derive;
use actix_web::{server, App, HttpRequest, Json, Result, http::Method};

#[derive(Serialize)]
struct MyObj {
    name: String,
}

fn getname(req: HttpRequest) -> Result<Json<MyObj>> {
    Ok(Json(MyObj{name: req.match_info().query("name")?}))
}

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    server::new(|| App::new()
        .resource("/", |r| r.f(index))
        .resource(r"/a/{name}", |r| r.method(Method::GET).f(getname))
    ).bind("127.0.0.1:8088").unwrap().run();
}


// fn main() {
//     App::new()
//         .resource(r"/a/{name}", |r| r.method(Method::GET).f(index))
//         .finish();
// }
