use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct Error1 {
    code: Code,
}

#[derive(Debug)]
struct Error2 {
    code: Code,
}

#[derive(Debug)]
struct Code {
    x: u32,
}

// fmt function is called for {} in print/write functions.
impl Display for Error1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Display Error1) Error1")
    }
}

impl Display for Error2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Display Error2) Error2")
    }
}

impl Error for Error1 {
    fn description(&self) -> &str {
        "Error::description: Error1"
    }

    fn cause(&self) -> Option<&Error> {
        Some(&self.code)
    }
}

impl Error for Error2 {
    fn description(&self) -> &str {
        "Error::description: Error2"
    }

    fn cause(&self) -> Option<&Error> {
        Some(&self.code)
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: Code = {}", self.x)
    }
}

impl Error for Code {
    fn description(&self) -> &str {
        "Error code"
    }
}

fn get_error2() -> Result<(), Error2> {
    match get_error1() {
        // e.code (which is error code of Error1) is the cause of Error2.
        Err(e) => Err(Error2 { code: e.code }),
        _ => Ok(()),
    }
}

fn get_error1() -> Result<(), Error1> {
    Err(Error1 {
        code: Code { x: 1 },
    })
}

fn print_error(mut err: &Error) {
    eprintln!("error: {:?}", err);
    while let Some(cause) = err.cause() {
        let _ = eprintln!("caused by: {:?}", cause);
        err = cause;
    }
}

fn main() {
    match get_error2() {
        Err(ref e) => {
            print_error(e);
            println!("-----");
            println!("Error: {}", e);
        }
        Ok(_) => println!("ok"),
    }
}
