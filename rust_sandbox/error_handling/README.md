Base on https://blog.burntsushi.net/rust-error-handling/

The ideal error handling is in "compose-custom-error-types.rs".

## The short story

Since this article is long, it is useful to have a quick summary for error
handling in Rust. These are my “rules of thumb.” They are emphatically not
commandments. There are probably good reasons to break every one of these
heuristics!

* If you’re writing short example code that would be overburdened by error
  handling, it’s probably just fine to use unwrap (whether that’s
  Result::unwrap, Option::unwrap or preferably Option::expect). Consumers of
  your code should know to use proper error handling. (If they don’t, send them
  here!)

* If you’re writing a quick ‘n’ dirty program, don’t feel ashamed if you use
  unwrap. Be warned: if it winds up in someone else’s hands, don’t be surprised
  if they are agitated by poor error messages!

* If you’re writing a quick ‘n’ dirty program and feel ashamed about panicking
  anyway, then you should probably use Box<Error> (or Box<Error + Send + Sync>)
  as shown in examples above. Another promising alternative is the anyhow crate
  and its anyhow::Error type. When using anyhow, your errors will automatically
  have backtraces attached to them when using nightly Rust.

* Otherwise, in a program, define your own error types with appropriate From and
  Error impls to make the ? operator macro more ergnomic.

* If you’re writing a library and your code can produce errors, define your own
  error type and implement the std::error::Error trait. Where appropriate,
  implement From to make both your library code and the caller’s code easier to
  write. (Because of Rust’s coherence rules, callers will not be able to impl
  From on your error type, so your library should do it.)

* Learn the combinators defined on Option and Result. Using them exclusively can
  be a bit tiring at times, but I’ve personally found a healthy mix of the ?
  operator and combinators to be quite appealing. and_then, map and unwrap_or
  are my favorites.
