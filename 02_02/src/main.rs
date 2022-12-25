// Ethan Meltzer
// 12/25/2022

use std::io::stdin;

fn main() {
    let mut total_sum:i32 = 0;
    let mut line = String::new();

    while stdin().read_line(&mut line).unwrap() != 0 {
        let p2:u8 = (line.chars().nth(0).unwrap() as u8) - ('A' as u8);
        // Now this is whether 0=lose, 1=draw, 2=win
        let p1:u8 = (line.chars().nth(2).unwrap() as u8) - ('X' as u8);

        total_sum += 1 + match p1 {
            0 => ((p2 + 2) % 3) + 3*p1,
            1 => p2 + 3*p1,
            2 => ((p2 + 1) % 3) + 3*p1,
            _ => panic!("Result is wrong!"),
        } as i32;
        
        line = "".to_string();
    };

    println!("{total_sum}");
}
