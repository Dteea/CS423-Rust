fn main() {
    let mut starting_row = [true, false, true, false, false, true, false, false];

    let mut row = [false; 8];

    for i in 0..9 {
        if i == 0 {
            for j in 0..8 {
                // If starting at index 0, we need to look at index 7, 0, and 1 and keep sliding until a row is completed.
                let temp = [
                    starting_row[(j + 7) % 8], // 7 % 8 = 7
                    starting_row[j % 8],       // 0 % 8 = 0
                    starting_row[(j + 1) % 8],
                ]; // 1 % 8 = 1

                //println!("{:?}", temp);
                row[j] = rule110(temp);
            }

            display_row(&starting_row);
        } else {
            for j in 0..8 {
                // If starting at index 0, we need to look at index 7, 0, and 1 and keep sliding until a row is completed.
                let temp = [
                    starting_row[(j + 7) % 8], // 7 % 8 = 7
                    starting_row[j % 8],       // 0 % 8 = 0
                    starting_row[(j + 1) % 8],
                ]; // 1 % 8 = 1

                //println!("{:?}", temp);
                row[j] = rule110(temp);
            }
        }

        display_row(&row);
        starting_row = row;
    }
}

fn rule110(bits: [bool; 3]) -> bool {
    match bits {
        [true, true, true] => false,
        [true, true, false] => true,
        [true, false, true] => true,
        [true, false, false] => false,
        [false, true, true] => true,
        [false, true, false] => true,
        [false, false, true] => true,
        [false, false, false] => false,
    }
}

// This function takes in a array ref of 8 bools and converts it into characters representing 0 and 1 and returns back the row
// after it is done borrowing.
fn display_row(row: &[bool; 8]) {
    let mut string = String::new();
    for bool in row {
        match bool {
            true => string.push('*'),
            false => string.push('.'),
        }
    }
    println!("{}", string);
    //row
}
