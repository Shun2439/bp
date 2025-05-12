extern crate iron;
extern crate router;

#[macro_use]
extern crate mime;

use rand::prelude::*;
use serde_json::json;

use iron::headers::ContentType;
use iron::prelude::*;
use iron::status;
use iron::{AfterMiddleware, Chain, Iron, IronResult, Request, Response};
use iron_cors::CorsMiddleware;
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

    fn random(&self) -> String {
        let mut rng = thread_rng();

        let choice = rng.gen_range(0, 6);

        match choice {
            0 => self.daikichi.clone(),
            1 => self.chuukichi.clone(),
            2 => self.kichi.clone(),
            3 => self.shoukichi.clone(),
            4 => self.suekichi.clone(),
            5 => self.kyou.clone(),
            _ => unreachable!("Error"),
        }
    }
}

struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("Hitting custom 404 middleware");

        if err.error.is::<NoRoute>() {
            let json_error = json!({
                "error": "NotFound",
                "message": "Therequested API endpoint does not exist."
            });
            let mut response = Response::with((status::NotFound, json_error.to_string()));
            response
                .headers
                .set(ContentType(mime!(Application/Json; Charset=Utf8)));
            Ok(response)
        } else {
            Err(err)
        }
    }
}

fn main() {
    let mut router = Router::new();
    router.get("/api/luck", handler, "get_luck");

    let cors_middleware = CorsMiddleware::with_allow_any();

    let mut chain = Chain::new(router);
    chain.link_around(cors_middleware);
    chain.link_after(Custom404);

    println!("Start server!");
    println!("Starting server on https://localhost:3000");
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn handler(_: &mut Request) -> IronResult<Response> {
    let luck = Luck::new();
    let random_luck_value = luck.random();

    let json_response_body = json!({
        "luck": random_luck_value
    });

    let mut response = Response::new();
    response.set_mut(status::Ok);
    response
        .headers
        .set(ContentType(mime!(Application/Json; Charset=Utf8)));
    response.set_mut(json_response_body.to_string());
    Ok(response)
}
