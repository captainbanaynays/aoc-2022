// Ethan Meltzer
// 01/03/2023

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
    let mut elf1:String = String::new(); 
    let mut elf2:String = String::new(); 
    let mut elf3:String = String::new();

    while stdin().read_line(&mut elf1).unwrap() != 0 &&
        stdin().read_line(&mut elf2).unwrap() != 0 &&
            stdin().read_line(&mut elf3).unwrap() != 0
            {
                trim_newline(&mut elf1);
                trim_newline(&mut elf2);
                trim_newline(&mut elf3);

                'a: for a in elf1.chars() {
                    for b in elf2.chars() {
                        for c in elf3.chars() {
                            if a == b && b == c {
                                if a.is_uppercase() {
                                    total += (a as u8 - 'A' as u8 + 27) as i32;
                                }
                                else {
                                    total += (a as u8 - 'a' as u8 + 1) as i32;
                                }
                                break 'a;
                    }
                }
            }
        }
        elf1 = "".to_string();
        elf2 = "".to_string();
        elf3 = "".to_string();
    }

   println!("{total}");
}
