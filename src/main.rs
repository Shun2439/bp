extern crate iron;
extern crate router;

#[macro_use]
extern crate mime;

use rand::prelude::*;

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

    fn random(&mut self) -> String {
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
            Ok(Response::with((status::NotFound, "Custom 404 response")))
        } else {
            Err(err)
        }
    }
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "example");

    let mut chain = Chain::new(router);
    chain.link_after(Custom404);

    println!("Start server!");
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn handler(_: &mut Request) -> IronResult<Response> {
    let mut luck = Luck::new();
    let random_luck_value = luck.random();

    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    let html_content = format!(
        r#"
                <h1> おみくじ </h1>
                今日の運勢は{}!
                "#,
        random_luck_value
    );
    response.set_mut(html_content);
    Ok(response)
}
