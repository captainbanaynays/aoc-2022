// Ethan Meltzer
// 01/14/2022

use std::io::stdin;

const MARKER_SIZE: usize = 4;

fn main() {
    let mut input: String = String::new();

    stdin().read_line(&mut input).unwrap();

    let input: Vec<char> = input.chars().collect();

    for i in 0..input.len() {
        let input_slice: &[char] = &input[(i..i + MARKER_SIZE)];

        let mut marker: bool = true;
        'j: for j in 0..input_slice.len() - 1 {
            for k in j + 1..input_slice.len() {
                if input_slice[j] == input_slice[k] {
                    marker = false;
                    break 'j;
                }
            }
        }

        if marker {
            println!("{}", i + input_slice.len());
            break;
        }
    }
}
