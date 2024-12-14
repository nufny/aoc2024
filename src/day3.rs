use regex::Regex;

fn get_mult_nums(input: &str) -> Vec<(u32,u32)> {

    let re = Regex::new(r"mul\((?<n>[0-9]+,[0-9]+)\)").unwrap();

    re.captures_iter(input).map(|caps| {
        let nums = caps.name("n").unwrap().as_str();
        nums
    })
    .map(|pair|pair.split_once(",").unwrap())
    .map(|(left,right)| (left.parse().expect("NaN"),right.parse().expect("NaN")))
    .collect()
}


pub mod p1 {
    use super::get_mult_nums;

    pub fn run(input: String) -> u32 {

        get_mult_nums(&input)
        .into_iter()
        .map(|(left,right)| left * right)
        .sum()
        
    }
}

pub mod p2 {
    pub fn run(_input: String) -> u32 {
        0
    }
}
