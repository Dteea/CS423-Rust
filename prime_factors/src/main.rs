use std::io;

fn main() {
    // Use only natural numbers.
    let mut input = String::new();

    // Experimenting with matching and looping until user enters valid input. Very cool
    loop {
        println!("Please enter a number: ");
        io::stdin().read_line(&mut input).expect("Read line fail.");

        match input.trim().parse() {
            Ok(value) => {
                println!("The prime factors of {} are: ", value);

                for elements in &factors(value) {
                    print!("{} ", elements);
                }
                break;
            }

            Err(_) => {
                println!("Input was not valid. Please enter a natural number.");
            }
        };
    }
}

fn factors(mut number: u64) -> Vec<u64> {
    let mut i = 2;
    let mut prime_factors = vec![];

    while i <= number {
        while number.is_multiple_of(i) {
            prime_factors.push(i);
            number /= i;
        }
        i += 1;
    }

    prime_factors
}

#[test]
fn no_factors() {
    let factors = factors(1);
    let expected = [];
    assert_eq!(factors, expected);
}

#[test]
fn prime_number() {
    let factors = factors(2);
    let expected = [2];
    assert_eq!(factors, expected);
}

#[test]
fn prime_number2() {
    let factors = factors(3);
    let expected = [3];
    assert_eq!(factors, expected);
}

#[test]
fn prime_number3() {
    let factors = factors(9);
    let expected = [3, 3];
    assert_eq!(factors, expected);
}

#[test]
fn prime_number4() {
    let factors = factors(4);
    let expected = [2, 2];
    assert_eq!(factors, expected);
}

#[test]
fn prime_number5() {
    let factors = factors(625);
    let expected = [5, 5, 5, 5];
    assert_eq!(factors, expected);
}
