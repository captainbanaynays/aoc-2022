// Ethan Meltzer
// 12/22/2022

use std::io::stdin;
use std::usize;

const ARRAY_SIZE:usize = 3;

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

fn main() {

    let mut single_sum = 0;
    let mut max_sums: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];
    let mut input_line = String::new();
    let mut read_ret;

    // Read in lines from input. Parse as integers, panic if error. Add to single sum.

    loop {

        read_ret = stdin()
            .read_line(&mut input_line)
            .unwrap();

        input_line = strip_trailing_newline(&input_line).to_string();

        if read_ret == 0 {
            break;
        }

        else if input_line == "" {
            for sum in max_sums.iter_mut() {
                if single_sum > *sum {
                   let temp:i32 = *sum;
                   *sum = single_sum;
                   single_sum = temp;
                }
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

    let sum:i32 = max_sums.iter().sum();
    println!("{sum}");
}

