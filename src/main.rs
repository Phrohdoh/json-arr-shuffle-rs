//! Shuffle items in a JSON array.
//!
//! This program requires input to come from stdin (piped) and prints the
//! shuffled JSON array to stdout with a trailing `\n` character.
//!
//! # Examples
//!
//! ```shell
//! $ echo "[5, 6, 7]" | json-arr-shuffle
//! [5,7,6]
//! ```
//!
//! ```shell
//! $ echo '["hello", 6, true]' | json-arr-shuffle
//! [6,"hello",true]
//! ```
//!

use std::{
    process,
    io::{
        self,
        Read as _,
    },
};

use rand::{
    seq::SliceRandom as _,
};

use serde_json::{
    Value,
};

fn main() {
    let mut stdin = io::stdin();
    let mut input = String::new();

    if let Err(e) = stdin.read_to_string(&mut input) {
        eprintln!("Failed to read from stdin: {:?}", e);
        process::exit(1);
    }

    let input = input.trim();

    let mut values = match serde_json::from_str(input) {
        Ok(Value::Array(vec)) => vec,
        Ok(val) => {
            eprintln!(
                "Failed to deserialize stdin as a JSON array (found {})",
                match val {
                    Value::Null => "null",
                    Value::Bool(_) => "bool",
                    Value::Number(_) => "number",
                    Value::Object(_) => "object",
                    Value::String(_) => "string",
                    Value::Array(_) => unreachable!(),
                },
            );
            process::exit(2);
        },
        Err(e) => {
            eprintln!("Failed to deserialize stdin as a JSON array: {:?}", e);
            process::exit(3);
        },
    };

    values.shuffle(&mut rand::thread_rng());
    let json = match serde_json::to_string(&values) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to serialize the shuffled values to JSON: {:?}", e);
            process::exit(4);
        },
    };

    println!("{}", json);
}
