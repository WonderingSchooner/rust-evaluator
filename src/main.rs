use std::fs;
use std::time::Instant;
use std::io::{self, BufRead};
use tokio_postgres::{NoTls, Error};

mod helper {
    include!("inbox/helper.rs");
}

async fn main() -> Result<(), Error> {
    // Connect to the database
    let (client, connection) =
    tokio_postgres::connect("host=localhost user=postgres password=secret dbname=rust_eval", NoTls).await?;

    // Spawn the connection to run in background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let file_path = "src/test_data.txt";

    // Read input string and expected output from the file
    let (input_string, expected_output) = match read_test_data(file_path) {
        Ok((input, output)) => (input, output),
        Err(e) => {
            eprintln!("❌ Failed to read test data: {}", e);
            return;
        }
    };

    let start = Instant::now();
    let message = helper::get_message(&input_string);
    let duration_ms = start.elapsed().as_millis() as i32;
    
    check_for_cheating(&expected_output);

    check_output(&expected_output, &message);

    println!("{}", message);
    println!("Execution time: {:.3?}", duration);
}

/// Reads the input string and expected output from a file
fn read_test_data(file_path: &str) -> io::Result<(String, String)> {
    let file = fs::File::open(file_path)?;
    let mut lines = io::BufReader::new(file).lines();

    let input_string = lines.next().unwrap_or(Ok(String::new()))?;
    let expected_output = lines.next().unwrap_or(Ok(String::new()))?;

    Ok((input_string, expected_output))
}

// ✅ Checks for cheating by seeing if the answer string is in helper.rs
fn check_for_cheating(expected_output: &str) {
    let helper_code = fs::read_to_string("src/inbox/helper.rs")
    .expect("Failed to read helper.rs");

    if helper_code.contains(expected_output) {
        println!("❌ Check failed: expected output found in helper.rs!");
    } else {
        println!("✅ Check passed: expected output NOT found in helper.rs!");
    }
}

// ✅ Check if the expected output and actual output are the same
fn check_output(expected_output: &str, actual_output: &str) {
    if expected_output == actual_output {
        println!("✅ Check passed: expected output and actual output match!");
    } else {
        println!("❌ Check failed: expected output and actual output do not match!");
    }
}