use clap::Parser;
use util::get_input;


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


fn main() {

    let args = Args::parse();
    let func = match args.day {
        1 => match args.part {
            1 => day1::p1::run,
            2 => day1::p2::run,
            _ => panic!()
        },
        2 => match args.part {
            1 => day2::p1::run,
            2 => day2::p2::run,
            _ => panic!()
        },
        3 => match args.part {
            1 => day3::p1::run,
            2 => day3::p2::run,
            _ => panic!()
        },
        4 => match args.part {
            1 => day4::p1::run,
            2 => day4::p2::run,
            _ => panic!()
        },
        5 => match args.part {
            1 => day5::p1::run,
            2 => day5::p2::run,
            _ => panic!()
        },
        
        // X => match args.part {
        //     1 => dayX::p1::run,
        //     2 => dayX::p2::run,
        //     _ => panic!()
        // }

        _ => panic!()
    };
    let input = get_input(args.day);
    let output = func(input);
    dbg!(output);

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



#[cfg(test)]
mod tests;

pub mod day1;

pub mod day2;

pub mod day3;

pub mod day4;

pub mod day5;
