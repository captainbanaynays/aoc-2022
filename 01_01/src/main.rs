// Ethan Meltzer
// 12/21/2022

use std::io;

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

fn main() {
    let mut single_sum = 0;
    let mut max_sum = 0;
    let mut input_line = String::new();
    let mut read_ret;

    // Read in lines from input. Parse as integers, panic if error. Add to single sum.

    loop {

        read_ret = io::stdin().read_line(&mut input_line).unwrap();
        input_line = strip_trailing_newline(&input_line).to_string();

        if read_ret == 0 {
            break;
        }
        else if input_line == "" {
            if single_sum > max_sum {
                max_sum = single_sum;
            }
            single_sum = 0;
        }
        else {
            single_sum += match input_line.parse::<i32>() {
                Ok(n) => n,
                Err(err) => panic!("Error reading the file: {err}"),
            }
        }
        input_line = "".to_string();
    }

    println!("{max_sum}");
}

