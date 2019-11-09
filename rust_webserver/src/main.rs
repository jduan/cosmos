extern crate iron;
// this anotation alerts Rust that we plan to use macros exported by the mime crate
#[macro_use]
extern crate mime;

use iron::prelude::*;
use iron::status;

fn main() {
    println!("Serving on http://localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    // The set_mut method uses its argument’s type to decide which part of the response to set
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        r#"
    <title>GCD Calculator</title>
    <form action="/gcd" method="post">
        <input type="text" name="n">
        <input type="text" name="n">
        <button type="submit">Compute GCD</button>
    </form>
    "#,
    );

    Ok(response)
}
