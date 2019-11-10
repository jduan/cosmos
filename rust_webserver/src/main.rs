extern crate iron;
// this anotation alerts Rust that we plan to use macros exported by the mime crate
#[macro_use]
extern crate mime;
extern crate router;
use router::Router;

use iron::prelude::*;
use iron::status;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
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

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    // The ::<UrlEncodedBody> part of the method call is a type parameter indicating which part of
    // the Request get_ref should retrieve.
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }

        Ok(map) => map,
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums,
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!(
                    "Value for 'n' parameter not a number: {:?}",
                    unparsed
                ));
                return Ok(response);
            }
            Ok(n) => numbers.push(n),
        }
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    let divisor = gcd(numbers[0], numbers[1]);
    response.set_mut(format!(
        "The greatest common divisor of the numbers is {}",
        divisor
    ));

    Ok(response)
}

fn gcd(m: u64, n: u64) -> u64 {
    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }
    if m == n {
        return m;
    }

    if m > n {
        gcd(m - n, n)
    } else {
        gcd(m, n - m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(14, gcd(98, 56));
        assert_eq!(5, gcd(5, 20));
    }
}
