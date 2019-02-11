extern crate constrain_value;

#[test]
fn test_calculate_simple() {
    assert_eq!(constrain_value::calculate_simple(5, 20, 1), 5);
    assert_eq!(constrain_value::calculate_simple(5, 20, 10), 10);
    assert_eq!(constrain_value::calculate_simple(5, 20, 25), 20);
}

#[test]
fn test_calculate_variable() {
    assert_eq!(constrain_value::calculate_variable(5, 100, 50, "5-"), 45);
    assert_eq!(constrain_value::calculate_variable(5, 100, 50, "5+"), 55);
    assert_eq!(constrain_value::calculate_variable(5, 100, 50, "50%-"), 25);
    assert_eq!(constrain_value::calculate_variable(5, 100, 50, "50%+"), 75);

    assert_eq!(constrain_value::calculate_variable(5, 100, 50, "100-"), 5);
    assert_eq!(constrain_value::calculate_variable(5, 100, 50, "100+"), 100);
}
