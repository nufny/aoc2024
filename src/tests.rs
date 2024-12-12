use super::util::*;


fn get_test_data(day: u8, part: u8) -> String {
    get_data(day, part, DataType::Test)
}

fn get_test_result_data(day: u8, part: u8) -> String {
    get_data(day, part, DataType::TestResult)
}

fn default_test(day: u8, part: u8, function: fn(&String) -> String) {
    let input = get_test_data(day, part);
    let result = function(&input);
    assert_eq!(result.to_string(), get_test_result_data(day, part))
}

use super::day1;
// Day 1 tests
#[test]
fn day1p1() {
    default_test(1, 1, |input| day1::p1::run(input.to_string()).to_string());
}
#[test]
fn day1p2() {
    default_test(1,2,|input| day1::p2::run(input.to_string()).to_string());
}

use super::day2;
// Day 2 tests
#[test]
fn day2p1() {
    default_test(2,1,|input| day2::p1::run(input.to_string()).to_string());
}
#[test]
fn day2p2() {
    default_test(2,2,|input| day2::p2::run(input.to_string()).to_string());
}
