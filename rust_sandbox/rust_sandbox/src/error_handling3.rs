/// https://nick.groenen.me/posts/rust-error-handling/
///
/// * thiserror: libraries should focus on producing meaningful, structured error types. This allows
/// applications to easily differentiate various error cases.
///
/// * anyhow: applications usually don't care about what error type your function returns, you just
/// want to be easy.
use anyhow::Result;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use thiserror::Error;

/// This version returns a Result with a dyn Error.
pub fn count_words<R: Read>(input: &mut R) -> Result<u32, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut wc = 0;
    for line in reader.lines() {
        for _word in line?.split_whitespace() {
            wc += 1;
        }
    }

    Ok(wc)
}

/// thiserror deliberately does not appear in your public API. You get the same thing as if you
/// had written an implementation of std::error::Error by hand.
#[derive(Error, Debug)]
pub enum WordCountError {
    #[error("Source contains no data")]
    EmptySource,

    #[error("Read error")]
    // "source" is used to keep the error chain (ie the context)
    ReadError { source: std::io::Error },

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

/// This version returns a Result with a dyn Error.
pub fn count_words2<R: Read>(input: &mut R) -> Result<u32, WordCountError> {
    let reader = BufReader::new(input);
    let mut wc = 0;
    for line in reader.lines() {
        // Wrap a low-level IO error into our library-level error
        let line = line.map_err(|source| WordCountError::ReadError { source })?;
        for _word in line.split_whitespace() {
            wc += 1;
        }
    }

    if wc == 0 {
        return Err(WordCountError::EmptySource);
    }
    Ok(wc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words() {
        let mut paragraph = "This string will be read".as_bytes();
        assert_eq!(5, count_words(&mut paragraph).unwrap());
    }

    /// This test works like application code. Think of it as the "main" function.
    #[test]
    fn test_count_words2() {
        let mut paragraph = "This string will be read".as_bytes();
        let wc = count_words2(&mut paragraph)
            .context(format!("unable to count words in '{:?}'", paragraph));
        assert_eq!(5, wc.unwrap());
    }
}
