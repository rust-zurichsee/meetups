extern crate iron;
extern crate router;

use iron::prelude::*;
use router::Router;

fn hello(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello")))
}

fn world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "World")))
}

fn request_logger(req: &mut Request) -> IronResult<()> {
    println!("{} {}", req.method, req.url);
    Ok(())
}

fn main() {
    let mut router = Router::new();
    router.get("/hello", hello, "hello");
    router.get("/world", world, "world");

    let mut chain = Chain::new(router);
    chain.link_before(request_logger);

    println!("Starting server on :3000");
    Iron::new(chain).http("localhost:3000").unwrap();
}
