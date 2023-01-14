// Ethan Meltzer
// 01/03/2023

use std::{char, collections::VecDeque, io::stdin, usize};

fn main() {
    let mut lines: VecDeque<String> = stdin().lines().map(|f| f.unwrap()).collect();
    let mut stack_lines: VecDeque<String> = VecDeque::new();

    let mut line: String = lines.pop_front().unwrap();
    while line.find(|c: char| c.is_ascii_digit()).is_none() {
        stack_lines.push_back(line);
        line = lines.pop_front().unwrap();
    }
    let nums: Vec<&str> = line.split_whitespace().collect();
    let num_columns: usize = nums[nums.len() - 1].parse().unwrap();

    let mut cargo_array: Vec<Vec<char>> = vec![Vec::new(); num_columns];

    for line in stack_lines.iter().rev() {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..num_columns {
            if chars[4 * i + 1].is_alphabetic() {
                cargo_array[i].push(chars[4 * i + 1]);
            }
        }
    }

    // Pop empty line between diagram and instructions
    lines.pop_front();

    // Now we have starting state, need to parse all directions
    while !(lines.is_empty()) {
        let line: String = lines.pop_front().unwrap();
        let line: Vec<&str> = line.split_whitespace().collect();
        let num_to_move: usize = line[1].parse().unwrap();
        let start_stack: usize = line[3].parse::<usize>().unwrap() - 1;
        let end_stack: usize = line[5].parse::<usize>().unwrap() - 1;
        let mut temp: Vec<char> = Vec::new();

        for _ in 0..num_to_move {
            temp.push(cargo_array[start_stack].pop().unwrap());
        }
        for _ in 0..num_to_move {
            cargo_array[end_stack].push(temp.pop().unwrap());
        }
    }

    // Now print last char in each vec as a String
    let mut print_string: String = String::new();

    for mut i in cargo_array {
        print_string.push(i.pop().unwrap());
    }

    println!("{print_string}");
}
