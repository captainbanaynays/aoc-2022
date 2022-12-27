// Ethan Meltzer
// 12/25/2022

use std::io::stdin;

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn main() {
   let mut total:i32 = 0;
   let mut line:String = String::new();

   while stdin().read_line(&mut line).unwrap() != 0 {
        trim_newline(&mut line);
        let line_len = line.len();
        // According to puzzle, this value must be even
        assert!(line_len % 2 == 0);
        
        let (bag1, bag2):(&str, &str) = line.split_at(line_len / 2);
        let bag1:String = bag1.to_string();
        let bag2:String = bag2.to_string();

        'ic: for c in bag1.chars() {
            for d in bag2.chars() {
                if c == d {
                    total += match c.is_uppercase() {
                        true => (c as u8 - 'A' as u8 + 27) as i32,
                        false => (c as u8 - 'a' as u8 + 1) as i32,
                    }; 
                    break 'ic
                }
            }
        }
        line = "".to_string();
   }
   println!("{total}");
}
