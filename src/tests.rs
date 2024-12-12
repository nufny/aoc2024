use super::util::*;
use super::day1;

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

/*
#[test]
fn dayxpy() {
    default_test(x,y,|input| dayx::py::out(input).to_string());
}
*/
#[test]
fn day1p1() {
    default_test(1, 1, |input| day1::p1::run(input.to_string()).to_string());
}
#[test]
fn day1p2() {
    default_test(1,2,|input| day1::p2::run(input.to_string()).to_string());
}