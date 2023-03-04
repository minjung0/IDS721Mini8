use std::io;
use std::cmp::{min, max};

fn main() {
    println!("Welcome to Min Max Game!");

    println!("Please enter a list of numbers separated by commas:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let nums: Vec<i32> = input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let result = min_max(nums);

    println!("The result is: {}", result);
}

fn min_max(mut nums: Vec<i32>) -> i32 {
    let mut n = nums.len()/2;
    
    while n > 0 {
        for i in 0..n {
            if i % 2 ==0 {
                nums[i] = min(nums[2 * i], nums[2 * i + 1]);
                println!("min: {}", nums[i]); 
            }
            else {
                nums[i] = max(nums[2 * i], nums[2 * i + 1]);
                println!("max: {}", nums[i]);
            }
        }
        n /= 2;
    }
    
    nums[0]
}