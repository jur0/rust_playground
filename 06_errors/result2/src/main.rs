struct TestError {
    code: i32,
}

type Result<T> = std::result::Result<T, TestError>;

// There is no need writing the error type.
fn get_result_err() -> Result<String> {
    Err(TestError { code: 10 })
}

fn get_result_ok() -> Result<String> {
    Ok("all good".to_string())
}

fn main() {
    let x = get_result_err().unwrap_or_else(|e| format!("error in x occured: {}", e.code));

    let y = get_result_ok().unwrap_or_else(|e| format!("error in y occured: {}", e.code));

    println!("x = {}", x);
    println!("y = {}", y);
}
