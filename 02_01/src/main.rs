// Ethan Meltzer
// 12/22/2022

use std::io::stdin;

// TODO: I fucked up the order of who goes first and who goes second? IDK need to check tomorrow.
fn main() {
    let mut total_sum:i32 = 0;
    let mut line = String::new();

    while stdin().read_line(&mut line).unwrap() != 0 {
        let p1:u8 = (line.chars().nth(0).unwrap() as u8) - ('A' as u8);
        let p2:u8 = (line.chars().nth(2).unwrap() as u8) - ('X' as u8);

        total_sum += p1 as i32 + match p2 {
            0 => 3*((p1 + 1) % 3),
            1 => 3*((p1 + 0) % 3),
            2 => 3*((p1 + 2) % 3),
            _ => panic!("Result is wrong!"),
        } as i32;
        
        line = "".to_string();
    };

    println!("{total_sum}");
}
