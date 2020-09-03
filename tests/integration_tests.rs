use lvp;

#[test]
fn addition() {
    assert_eq!(8, lvp::solutions::stack_solution::longest_valid_parentheses(String::from("((()()))")));
}
