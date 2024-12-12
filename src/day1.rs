pub mod p1 {

    pub fn run(input: String) -> u32 {
        let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
            .lines()
            .into_iter()
            .map(|e| {
                e.split_once("   ").expect("Invalid Input(Wrong Syntax)")
            })
            .map(|(left, right)| {
                (
                    left.parse::<u32>().expect("Invalid Input(NaN)"),
                    right.parse::<u32>().expect("Invalid Input(NaN)"),
                )
            })
            .unzip();
        left.sort();
        right.sort();
        left.into_iter()
            .zip(right)
            .map(|pair| {
                let (left, right) = pair;
                let out: u32 = {
                    if left < right {
                        right - left
                    } else {
                        left - right
                    }
                };
                out
            })
            .sum()
    }
}

pub mod p2 {
    pub fn run(input: String) -> u32 {
        let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .into_iter()
        .map(|e| {
            e.split_once("   ").expect("Invalid Input(Wrong Syntax)")
        })
        .map(|(left, right)| {
            (
                left.parse::<u32>().expect("Invalid Input(NaN)"),
                right.parse::<u32>().expect("Invalid Input(NaN)"),
            )
        })
        .unzip();

        left.iter().map(|num| right.iter().filter(|e| *e == num).count() as u32 * num).sum()
        
    }
}
