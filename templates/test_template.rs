
// Day x tests
#[test]
fn dayxp1() {
    default_test(x,1,|input| dayx::p1::out(input).to_string());
}
#[test]
fn dayxp2() {
    default_test(x,2,|input| dayx::p2::out(input).to_string());
}
