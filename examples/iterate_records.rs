extern crate fit_no_io;

use std::path::PathBuf;
use fit_no_io::{self as fit, MessageType};

fn main() {
    let filepath = PathBuf::from("data/garmin_1000.fit");
    let msgs = fit::run(&filepath);
    for r in msgs.iter().filter(|m| m.kind == MessageType::Record) {
        println!("{:?}", r.values);
    }
}
