#[cfg(test)]
mod tests {
    use std::process::Command;

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
}
