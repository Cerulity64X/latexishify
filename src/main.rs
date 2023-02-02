use std::{io::{stdin, BufRead}, env::args};

use latexishify::latexishify;

fn main() {
    let mut argv = args();
    argv.next();
    let mut had_arguments = false;
    while let Some(arg) = argv.next() {
        had_arguments = true;
        println!("{}", latexishify(&arg));
    }
    if !had_arguments {
        for i in stdin().lock().lines() {
            if let Ok(inp) = i {
                println!("{}", latexishify(&inp));
            }
        }
    }
}
