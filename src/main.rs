extern crate pencil;
extern crate motivations;
extern crate pick_one;

use pencil::{Pencil, PencilResult, Request, Response};

fn hello_handler(_: &mut Request) -> PencilResult {
   let random_choice = pick_one::pick_one_str(&motivations::MOTIVATIONS); 
   Ok(Response::from(random_choice))
}

fn main() {
    let mut app = Pencil::new("/src");

    app.get("/", "hello", hello_handler);


    let host = "127.0.0.1";
    let port = "7878";
    let address = format!("{}:{}", host, port);

    println!("* Started server at http://{}", address);
    app.run(address);
}
