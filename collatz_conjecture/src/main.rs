use std::io;
fn main() {
    // Use only positive numbers.
    let mut input = String::new();

    // Great example for myself for dealing with input and looping back to the beginning until good input is received.
    loop {
        println!("Please enter a positive number: ");
        io::stdin().read_line(&mut input).expect("Read line fail.");

        match input.trim().parse() {
            Ok(value) => {
                let temp = value;
                println!(
                    "{} took {:?} step to get to 1.",
                    temp,
                    collatz_conjecture(value)
                );
                break;
            }

            Err(_) => {
                println!("Input was not valid. Please enter a positive number.");
                input.clear();
            }
        };
    }
}

fn collatz_conjecture(mut number: u64) -> Option<u64> {
    if number == 0 {
        println!("I don't know how you got past all the checking...");
        return None;
    }
    let mut steps = 0;

    loop {
        match number % 2 {
            // Even case
            0 => {
                if number == 1 {
                    return Some(steps);
                }

                number /= 2;
                steps += 1;
            }

            // Odd case
            1 => {
                if number == 1 {
                    return Some(steps);
                }

                number = (number * 3) + 1;
                steps += 1;
            }

            _ => {
                println!("I don't know how you got past all the type checking...");
                return None;
            }
        }
    }
}

// Tests
#[test]
fn zero_steps_for_one() {
    let out = collatz_conjecture(1);
    let expected = Some(0);
    assert_eq!(out, expected);
}
#[test]
fn divide_if_even() {
    let out = collatz_conjecture(16);
    let expected = Some(4);
    assert_eq!(out, expected);
}
#[test]
fn even_and_odd_steps() {
    let out = collatz_conjecture(12);
    let expected = Some(9);
    assert_eq!(out, expected);
}
#[test]
fn larger_number_of_even_and_odd_steps() {
    let out = collatz_conjecture(1_000_000);
    let expected = Some(152);
    assert_eq!(out, expected);
}
#[test]
fn zero_is_an_error() {
    let out = collatz_conjecture(0);
    let expected = None;
    assert_eq!(out, expected);
}
