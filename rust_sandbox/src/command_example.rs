use std::process::Command;

pub fn run_command() -> std::io::Result<()> {
    let status = Command::new("ls").arg("/tmp").status()?;
    println!("command status: {:?}", status);

    Ok(())
}
