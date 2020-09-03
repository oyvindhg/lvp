use std::cmp::max;

#[derive(Debug)]
struct ActiveParenthesis{
    start_idx: i32,
    previous_length: i32
}

///#Longest valid parentheses
///Find the length of the longest valid parentheses in a string of only '(' and ')'
///```
/// let result = lvp::solutions::stack_solution::longest_valid_parentheses(String::from("((()()))"));
/// assert_eq!(8, result);
/// ```
pub fn longest_valid_parentheses(s: String) -> i32 {
    let mut longest: i32 = 0;
    let mut current_length: i32 = 0;
    let mut active_parentheses: Vec<ActiveParenthesis> = vec![];
    let chars = s.chars();
    for (i, char) in chars.enumerate() {
        match char {
            '(' => {
                active_parentheses.push(ActiveParenthesis{start_idx: i as i32, previous_length: current_length});
                current_length = 0;
            },
            ')' => {
                if !active_parentheses.is_empty() {
                    current_length = i as i32 + 1 + active_parentheses.last().unwrap().previous_length - active_parentheses.last().unwrap().start_idx;
                    longest = max(
                        current_length,
                        longest
                    );
                    active_parentheses.pop();
                }
                else {
                    current_length = 0;
                }
            },
            _ => panic!("Illegal characters")
        }
        //println!("iter: {}, par: {:?}, length: {}, max: {}", i, parentheses, current_length, longest);
    }
    return longest;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, longest_valid_parentheses(String::from("()()")));
    }

    #[test]
    fn test_2() {
        assert_eq!(8, longest_valid_parentheses(String::from("((()()))")));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, longest_valid_parentheses(String::from(")))(((")));
    }

    #[test]
    fn test_4() {
        assert_eq!(2, longest_valid_parentheses(String::from("()(()(((")));
    }

    #[test]
    fn test_5() {
        assert_eq!(0, longest_valid_parentheses(String::from("")));
    }

    #[test]
    #[should_panic(expected = "Illegal characters")]
    fn test_6() {
        longest_valid_parentheses(String::from("(((;)))"));
    }
}
