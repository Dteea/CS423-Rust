use std::io;

fn main() {
    // Could move this into a function to handle input to test to make neater.
    // Sum only natural numbers not including 0.
    let mut input = String::new();

    println!("Please enter a number: ");
    io::stdin().read_line(&mut input).expect("Read line fail.");

    let number: u32 = input.trim().parse().expect("Input is not natural number.");

    let difference = square_of_sum(number) - sum_of_squares(number);
    println!("square_of_sum is: {}", square_of_sum(number));
    println!("sum_of_squares is: {}", sum_of_squares(number));
    println!(
        "The difference of squaring the sum and summing the squares up to {} is: {}",
        number, difference
    );
}

fn square_of_sum(n: u32) -> u32 {
    let mut count = 1;
    let mut total = 0;

    while count <= n {
        total += count;
        count += 1;
    }

    total.pow(2)
}

fn sum_of_squares(n: u32) -> u32 {
    let mut count = 1;
    let mut total = 0;

    while count <= n {
        total += count.pow(2);
        count += 1;
    }

    total
}

// Testing!
#[test]
fn test_square_of_nums() {
    assert_eq!(square_of_sum(10), 3025);
    assert_eq!(square_of_sum(20), 44100);
    assert_eq!(square_of_sum(186), 302446881);
}

#[test]
fn test_sum_of_squares() {
    assert_eq!(sum_of_squares(10), 385);
    assert_eq!(sum_of_squares(20), 2870);
    assert_eq!(sum_of_squares(186), 2162281);
}
