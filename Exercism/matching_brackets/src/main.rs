use std::io;
fn main() {
    let mut input = String::new();
    println!("Please enter a string containing brackets: ");
    io::stdin().read_line(&mut input).expect("Read line fail.");

    does_string_have_matching_brackets(input);
}

/// This function takes in a string as an argument and loops through each character, checking to see if brackets are found.
/// It utilizes a vector to store the current open brackets with a push and pop method and depending on if it finds a match, will be 
/// popped from the vector. It returns true if all brackets are matching and false if not.
fn does_string_have_matching_brackets(input: String) -> bool {
    let mut order_of_brackets = Vec::new();
    for char in input.trim().chars() {
        match char {
            '{' | '[' | '(' => {
                //print!("{}", char);
                order_of_brackets.push(char);
            }

            '}' => match order_of_brackets.last() {
                Some(value) => {
                    if *value == '{' {
                        order_of_brackets.pop();
                    } else {
                        return false;
                    }
                }
                None => {
                    order_of_brackets.push('}');
                }
            },

            ']' => match order_of_brackets.last() {
                Some(value) => {
                    if *value == '[' {
                        order_of_brackets.pop();
                    } else {
                        return false;
                    }
                }
                None => {
                    order_of_brackets.push(']');
                }
            },

            ')' => match order_of_brackets.last() {
                Some(value) => {
                    if *value == '(' {
                        order_of_brackets.pop();
                    } else {
                        return false;
                    }
                }
                None => {
                    order_of_brackets.push(')');
                }
            },

            _ => {
                continue;
            }
        }
    }
    if order_of_brackets.is_empty() {
        print!("all brackets match");
    } else {
        println!("not all brackets match");
    }
    order_of_brackets.is_empty()
}

// Test just taken and modified from Exercism
#[test]
fn paired_square_brackets() {
    let test_str = String::from("[]");
    assert!(does_string_have_matching_brackets(test_str));
}
#[test]
fn empty_string() {
    let test_str = String::from("");
    assert!(does_string_have_matching_brackets(test_str));
}
#[test]
fn unpaired_brackets() {
    let test_str = String::from("[[");
    assert!(!does_string_have_matching_brackets(test_str));
}
#[test]
fn wrong_ordered_brackets() {
    let test_str = String::from("}{");
    assert!(!does_string_have_matching_brackets(test_str));
}
#[test]
fn wrong_closing_bracket() {
    let test_str = String::from("{]");
    assert!(!does_string_have_matching_brackets(test_str));
}
#[test]
fn paired_with_whitespace() {
    let test_str = String::from("{ }");
    assert!(does_string_have_matching_brackets(test_str));
}
#[test]
fn partially_paired_brackets() {
    let test_str = String::from("{[])");
    assert!(!does_string_have_matching_brackets(test_str));
}
#[test]
fn simple_nested_brackets() {
    let test_str = String::from("{[]}");
    assert!(does_string_have_matching_brackets(test_str));
}
#[test]

fn several_paired_brackets() {
    let test_str = String::from("{}[]");
    assert!(does_string_have_matching_brackets(test_str));
}
#[test]

fn paired_and_nested_brackets() {
    let test_str = String::from("([{}({}[])])");
    assert!(does_string_have_matching_brackets(test_str));
}
#[test]

fn unopened_closing_brackets() {
    let test_str = String::from("{[)][]}");
    assert!(!does_string_have_matching_brackets(test_str));
}
#[test]

fn unpaired_and_nested_brackets() {
    let test_str = String::from("([{])");
    assert!(!does_string_have_matching_brackets(test_str));
}
#[test]

fn paired_and_wrong_nested_brackets() {
    let test_str = String::from("[({]})");
    assert!(!does_string_have_matching_brackets(test_str));
}
#[test]

fn paired_and_wrong_nested_brackets_but_innermost_are_correct() {
    let test_str = String::from("[({}])");
    assert!(!does_string_have_matching_brackets(test_str));
}
#[test]

fn paired_and_incomplete_brackets() {
    let test_str = String::from("{}[");
    assert!(!does_string_have_matching_brackets(test_str));
}
#[test]

fn too_many_closing_brackets() {
    let test_str = String::from("[]]");
    assert!(!does_string_have_matching_brackets(test_str));
}
#[test]

fn early_unexpected_brackets() {
    let test_str = String::from(")()");
    assert!(!does_string_have_matching_brackets(test_str));
}
#[test]

fn early_mismatched_brackets() {
    let test_str = String::from("{)()");
    assert!(!does_string_have_matching_brackets(test_str));
}
#[test]

fn math_expression() {
    let test_str = String::from("(((185 + 223.85) * 15) - 543)/2");
    assert!(does_string_have_matching_brackets(test_str));
}
#[test]

fn complex_latex_expression() {
    let test_str = String::from(
        "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \\end{array}\\right)",
    );
    assert!(does_string_have_matching_brackets(test_str));
}
