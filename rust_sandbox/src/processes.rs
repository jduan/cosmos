#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::io::{Read, Write};
    use std::process::{Command, Stdio};

    static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";

    #[test]
    fn get_output_of_external_process_success() {
        let output = Command::new("rustc")
            .arg("--version")
            .output()
            .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));

        assert!(output.status.success());
        let s = String::from_utf8_lossy(&output.stdout);
        println!("rustc succeeded and stdout was:\n{}", s);
    }

    #[test]
    fn get_output_of_external_process_failure() {
        let output = Command::new("rustc")
            .arg("--badarg")
            .output()
            .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));

        assert!(!output.status.success());
        let s = String::from_utf8_lossy(&output.stderr);
        println!("rustc failed and stderr was:\n{}", s);
    }

    #[test]
    /// The "std::Child" struct represents a running child process, and exposes the "stdin",
    /// "stdout", and "stdout" handles for interaction with the underlying process via pipes.
    fn interact_with_process_via_pipes() {
        let process = match Command::new("wc")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(why) => panic!("couldn't spawn wc: {}", why.description()),
            Ok(proc) => proc,
        };

        match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
            Err(why) => panic!("couldn't write to wc stdin: {}", why.description()),
            Ok(_) => println!("sent pangram to wc"),
        }

        let mut s = String::new();
        match process.stdout.unwrap().read_to_string(&mut s) {
            Err(why) => panic!("couldn't read wc stdout: {}", why.description()),
            Ok(_) => {
                println!("wc responded with:\n{}", s);
                let counts: Vec<u32> = s
                    .trim()
                    .split_whitespace()
                    .into_iter()
                    .map(|count| count.parse::<u32>().unwrap())
                    .collect();
                assert_eq!(vec![1, 9, 45], counts);
            }
        }
    }
}
