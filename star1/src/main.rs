use std::io::{self,Error,Read};

fn main() -> Result<(),Error> {
    println!("hi");
    let mut input = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut input).expect("oops");

    let nums: Vec<_> = input.lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let res = nums.iter().zip(nums.iter().skip(1))
        .filter(|(a,b)| a < b)
        .count();

    println!("{}", res);
    Ok(())
}
