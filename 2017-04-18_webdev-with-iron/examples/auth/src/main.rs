extern crate iron;

use iron::prelude::*;
use iron::headers::Authorization;
use iron::typemap::Key;

#[derive(Debug)]
struct User {
    username: String,
}

impl Key for User { type Value = User; }

fn user_middleware(req: &mut Request) -> IronResult<()> {
    if let Some(header) = req.headers.get::<Authorization<String>>() {
        let user = User { username: header.0.clone() };
        req.extensions.insert::<User>(user);
    };
    Ok(())
}

fn hello_world(req: &mut Request) -> IronResult<Response> {
    let text = match req.extensions.get::<User>() {
        Some(user) => format!("Hello, {}", user.username),
        None => format!("Hello, world"),
    };
    Ok(Response::with((iron::status::Ok, text)))
}

fn main() {
    let mut chain = Chain::new(hello_world);
    chain.link_before(user_middleware);
    println!("Starting server on :3000");
    Iron::new(chain).http("localhost:3000").unwrap();
}
