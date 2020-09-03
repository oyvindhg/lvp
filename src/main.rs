mod solutions;

fn main() {
    println!("{}", solutions::stack_solution::longest_valid_parentheses(String::from("((()()))")));
}
