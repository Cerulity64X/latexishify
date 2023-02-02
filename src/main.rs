use std::io::{stdin, BufRead};

use latexishify::latexishify;

fn main() {
    for i in stdin().lock().lines() {
        if let Ok(inp) = i {
            println!("{}", latexishify(&inp));
        }
    }
}
