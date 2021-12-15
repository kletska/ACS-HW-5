use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Mutex, Arc};

use app::run_app;

use crate::app::parse_args;
use crate::programmer::Writer;

mod task;
mod chat;
mod command;
mod programmer;
mod app;


fn main() {
    println!("The program started");

    let config = match parse_args() {
        Ok(config) => config,
        Err(_) => {
            println!("wrong command line arguments format");
            return;
        }
    };

    let input: Box<dyn BufRead> = match config.input {
        Some(s) => match File::open(s) {
            Ok(res) => Box::new(BufReader::new(res)),
            Err(_) => {
                Box::new(BufReader::new(std::io::stdin()))
            }
        }
        None => {
            Box::new(BufReader::new(std::io::stdin()))
        }
    };

    let output: Writer = match config.output {
        Some(s) => match File::create(s) {
            Ok(res) => {
                Arc::new(Mutex::new(res))
            }
            Err(_) => {
                Arc::new(Mutex::new(std::io::stdout()))
            }
        }
        None => {
            Arc::new(Mutex::new(std::io::stdout()))
        }
    };


    if let Err(err) = run_app(config.n, input, output) {
        println!("error occured while running\n The error message: {}", err);
    }
    println!("The program successfully finished");
}
