use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::{self, Write};
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn file_doesnt_exist() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::main_binary()?;
    cmd.arg("-p foobar").arg("-f test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    let content = "A test\n\
                   Actual content\n\
                   More content\n\
                   Another test\n";
    writeln!(file, "{}", content)?;
    let mut cmd = Command::main_binary()?;
    cmd.arg("-p test").arg("-f").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("test\nAnother test"));

    Ok(())
}

#[test]
// When searching for an empty pattern, it should return all the lines.
fn empty_pattern() -> Result<(), Box<std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    let content = "First line\n\
                   Second line";
    writeln!(file, "{}", content)?;
    let mut cmd = Command::main_binary()?;
    cmd.arg("-p").arg("").arg("-f").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("First line\nSecond line"));

    Ok(())
}
