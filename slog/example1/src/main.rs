#[macro_use]
extern crate slog;

use slog::*;
use std::{fmt, result};

struct PrintSerializer;

impl Serializer for PrintSerializer {
    fn emit_arguments(&mut self, key: Key, val: &fmt::Arguments) -> Result {
        print!("{}={}", key, val);
        Ok(())
    }
}

struct PrintDrain;

// Implementation of a simple Drain that prints log messages.
impl Drain for PrintDrain {
    type Ok = ();
    type Err = ();

    fn log(&self, record: &Record, values: &OwnedKVList) -> result::Result<Self::Ok, Self::Err> {
        print!("{}", record.msg());

        record.kv().serialize(record, &mut PrintSerializer).unwrap();
        values.serialize(record, &mut PrintSerializer).unwrap();

        println!("");

        Ok(())
    }
}

fn main() {
    let log = Logger::root(Fuse(PrintDrain), o!("v" => "0.0.1"));

    info!(log, "App started!");
    warn!(log, "Warning!");
}
