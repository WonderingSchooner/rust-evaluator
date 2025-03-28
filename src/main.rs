use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::time::Instant;
use std::path::Path;

/// Reads Rust code from the inbox/main.rs file
fn read_code_from_inbox() -> io::Result<String> {
    let path = "inbox/main.rs";
    fs::read_to_string(path)
}

/// Compiles the given Rust code file
fn compile_code(source_file: &str, output_file: &str) -> io::Result<bool> {
    let output = Command::new("rustc")
        .arg(source_file)
        .arg("-o")
        .arg(output_file)
        .stderr(Stdio::piped())
        .output()?;

    if !output.stderr.is_empty() {
        eprintln!("Compilation Error:\n{}", String::from_utf8_lossy(&output.stderr));
        return Ok(false);
    }
    Ok(true)
}

/// Executes the compiled binary and returns execution time
fn execute_code(binary: &str) -> io::Result<Option<std::time::Duration>> {
    let start_time = Instant::now();
    let execution = Command::new(binary)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    
    if execution.success() {
        Ok(Some(start_time.elapsed()))
    } else {
        eprintln!("Execution failed");
        Ok(None)
    }
}

fn main() {
    match read_code_from_inbox() {
        Ok(code) => {
            let file_path = "inbox/main.rs";
            let binary_path = "inbox/main";

            if compile_code(file_path, binary_path).unwrap_or(false) {
                if let Ok(Some(duration)) = execute_code(binary_path) {
                    println!("Execution time: {:.3?}", duration);
                }
            }
        }
        Err(e) => eprintln!("Failed to read code: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_code_from_inbox() {
        let test_code = "fn main() { println!(\"Hello, test!\"); }";
        let path = "inbox/main.rs";

        // Ensure the inbox directory exists
        fs::create_dir_all("inbox").unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(test_code.as_bytes()).unwrap();

        let code = read_code_from_inbox().unwrap();
        assert_eq!(code, test_code);

        // Cleanup
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_compile_code_success() {
        let test_code = "fn main() { println!(\"Hello, compile test!\"); }";
        let path = "inbox/main.rs";
        let binary_path = "inbox/main";

        fs::create_dir_all("inbox").unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(test_code.as_bytes()).unwrap();

        let result = compile_code(path, binary_path).unwrap();
        assert!(result);

        // Cleanup
        fs::remove_file(path).unwrap();
        fs::remove_file(binary_path).unwrap();
    }

    #[test]
    fn test_compile_code_failure() {
        let test_code = "fn main() { syntax error! }"; // Invalid Rust
        let path = "inbox/main.rs";

        fs::create_dir_all("inbox").unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(test_code.as_bytes()).unwrap();

        let result = compile_code(path, "inbox/main").unwrap();
        assert!(!result);

        // Cleanup
        fs::remove_file(path).unwrap();
    }
}
