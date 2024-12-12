// use clap::Parser;
use util::get_input;

/* 
#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// AOC day 1<=n<=25
    #[arg(short, long)]
    day: u8,

    /// part 1 or 2
    #[arg(short, long, default_value_t = 1)]
    part: u8,
} 
 */

fn main() {
    let input = get_input(1);
    let output = day1::p2::run(input);
    dbg!(output);

    // let args = Args::parse();
    // let day_module = match args.day {
    //     1 => day1,
    //     _ => panic!()
    // };
    // let part_module = day_module::p{args.part};
}


pub mod util {
    pub enum DataType {
        Input,
        Test,
        TestResult,
    }

    pub fn get_data(day: u8, part: u8, data_type: DataType) -> String {
        let filename = match data_type {
            DataType::Input => "input",
            DataType::Test => "test",
            DataType::TestResult => "test_result",
        };
        let path = match data_type {
            DataType::Input | DataType::Test => format!("inputs/{day}/{filename}.txt"),
            DataType::TestResult => format!("inputs/{day}/{part}/{filename}.txt"),
        };
        std::fs::read_to_string(path).unwrap()
    }

    pub fn get_input(day:u8) -> String {
        get_data(day, 1, DataType::Input)
    }
}

pub mod day1;

//pub mod day2;

//pub mod day3;

#[cfg(test)]
mod tests;
