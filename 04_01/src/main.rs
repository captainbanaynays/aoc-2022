// Ethan Meltzer
// 01/03/2023

use std::io::stdin;
use std::vec::Vec;

fn main() {
    let mut total:i32 = 0;
    let mut line:String = String::new();

    while stdin().read_line(&mut line).unwrap() != 0 {
        line = line.trim().to_string();
        let slices = line.split(&['-', ','][..]);
        let mut nums:Vec<i32> = Vec::new();

        for i in slices {
            nums.push(i.parse::<i32>().unwrap());
        }

        assert!(nums.len() == 4);
        assert!(nums[1] >= nums[0] && nums[3] >= nums[2]);

        // Case 1: elf1 contains elf2
        if (nums[0] <= nums[2] && nums[1] >= nums[3]) ||
        // Case 2: elf2 contains elf1
            (nums[0] >= nums[2] && nums[1] <= nums[3])
        {
            total += 1;
        }
        
        line = "".to_string();
    }
    println!("{total}");
}

