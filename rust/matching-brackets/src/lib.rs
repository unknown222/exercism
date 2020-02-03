use std::ops::Index;

pub fn brackets_are_balanced(string: &str) -> bool {
    let brackets = ['[', ']', '(', ')', '{', '}'];
    let mut left_bracket_stack: Vec<char> = Vec::new();

    for c in string.chars() {
        let index = brackets.iter().position(|&b| c == b );
        if index == None {
            continue;
        }

        if index.unwrap() % 2 == 0 {
            left_bracket_stack.push(c);
            continue;
        }

        let left_bracket = left_bracket_stack.pop();

        if left_bracket == Some(brackets[index.unwrap() - 1]) {
            continue;
        } else {
            return false;
        }
    }

    if left_bracket_stack.len() != 0 {
        return false;
    }

    return true;
}
