extern crate iron;
extern crate router;

#[macro_use]
extern crate mime;

use iron::prelude::*;
use iron::status;
use iron::{AfterMiddleware, Chain, Iron, IronResult, Request, Response};
use router::{NoRoute, Router};

struct Luck {
    daikichi: String,
    chuukichi: String,
    kichi: String,
    shoukichi: String,
    suekichi: String,
    kyou: String,
}

impl Luck {
    fn new() -> Self {
        Luck {
            daikichi: String::from("大吉"),
            chuukichi: String::from("中吉"),
            kichi: String::from("吉"),
            shoukichi: String::from("小吉"),
            suekichi: String::from("末吉"),
            kyou: String::from("凶"),
        }
    }
}

struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("Hitting custom 404 middleware");

        if err.error.is::<NoRoute>() {
            Ok(Response::with((status::NotFound, "Custom 404 response")))
        } else {
            Err(err)
        }
    }
}

fn main() {
    let luck = Luck::new();
    println!("{}", luck.daikichi);

    let mut router = Router::new();
    router.get("/", handler, "example");

    let mut chain = Chain::new(router);
    chain.link_after(Custom404);

    println!("Start server!");
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn handler(_: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        r#"
                <h1> demo </h1>
                "#,
    );
    Ok(response)
}
