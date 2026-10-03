use std::io;

fn main() {
    let mut input = String::new();

    println!("Please enter something like a word: ");
    // Very cool functions, need to lookup how to use them and what they do. 
    io::stdin().read_line(&mut input).unwrap();

    // Chain String functions to reverse!
    let reverse_input: String = input.chars().rev().collect();

    println!("Your reversed input is: {}", reverse_input);

}

// Tests
// Using some of the test examples.
#[test]
fn empty_string() {
    let input = "";
    let reversed: String = input.chars().rev().collect();
    assert_eq!(input, reversed);
}

#[test]
fn random_string() {
    let input = "thunder";
    let reversed: String = input.chars().rev().collect();
    let answer = "rednuht";
    assert_eq!(answer, reversed);
}


#[test]
fn string_with_capitals() {
    let input = "HaMmEr";
    let reversed: String = input.chars().rev().collect();
    let answer = "rEmMaH";
    assert_eq!(answer, reversed);
}

#[test]
fn string_palindrome() {
    let input = "civic";
    let reversed: String = input.chars().rev().collect();
    assert_eq!(input, reversed);
}

#[test]
fn string_numbers() {
    let input = "12345";
    let reversed: String = input.chars().rev().collect();
    let answer = "54321";
    assert_eq!(answer, reversed);
}