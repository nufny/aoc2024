enum Order {
    Ascending,
    Descending,
}

struct Report {
    nums: Vec<u32>,
}
impl Report {
    pub fn is_valid(&self) -> bool {
        let mut order= None;
        for window in self.nums.windows(2){
            match window[0] as i64 - window[1] as i64 {
                1..=3 => {
                    match order {
                        // 9 - 8 = 1
                        Some(Order::Descending) => (),
                        Some(Order::Ascending) => return false,
                        None => order = Some(Order::Descending),
                    }
                },
                -3..=-1 => {
                    match order {
                        // 1 - 2 = -1
                        Some(Order::Ascending) => (),
                        Some(Order::Descending) => return false,
                        None => order = Some(Order::Ascending),
                    }
                },
                _ => return false
            }

        };
        true
    }
    pub fn is_valid_with_dampener(&self) -> bool {

        if !check_nums(&self.nums){
            return (0..self.nums.len())
            .map(|index|  [self.nums[0..index].to_vec(),self.nums[index+1..].to_vec()].concat())
            .filter(|permutation| check_nums(permutation))
            .count() > 0
        }
        true
    }
}

fn check_nums(nums: &Vec<u32>) -> bool {
    let mut order= None;
        for window in nums.windows(2){
            match window[0] as i64 - window[1] as i64 {
                1..=3 => {
                    match order {
                        // 9 - 8 = 1
                        Some(Order::Descending) => (),
                        Some(Order::Ascending) => return false,
                        None => order = Some(Order::Descending),
                    }
                },
                -3..=-1 => {
                    match order {
                        // 1 - 2 = -1
                        Some(Order::Ascending) => (),
                        Some(Order::Descending) => return false,
                        None => order = Some(Order::Ascending),
                    }
                },
                _ => return false
            }

        };
        true
}

pub mod p1 {
    use super::Report;
    pub fn run(input: String) -> u32 {
        let valid_reports = input
        .split("\n")
        .filter(|val| !val.is_empty())
        .map(
            |report| 
            Report{
                nums:  report.split_whitespace()
                .map(|num|num.parse::<u32>()
                .expect("Error NaN"))
                .collect()
            }
        ).filter(|report| report.is_valid());
        valid_reports.count() as u32
    }
}

pub mod p2 {
    use super::Report;
    pub fn run(input: String) -> u32 {
        let valid_reports = input
        .split("\n")
        .filter(|val| !val.is_empty())
        .map(
            |report| 
            Report{
                nums:  report.split_whitespace()
                .map(|num|num.parse::<u32>()
                .expect("Error NaN"))
                .collect()
            }
        ).filter(|report| report.is_valid_with_dampener());
        valid_reports.count() as u32
    }
}
