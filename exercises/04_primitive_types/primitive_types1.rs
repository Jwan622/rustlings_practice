// Booleans (`bool`)

fn main() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    // TODO: Define a boolean variable with the name `is_evening` before the `if` statement below.
    // The value of the variable should be the negation (opposite) of `is_morning`.
    // let is_evening
    let is_evening: bool = !is_morning;
    println!("is_evening: {}", is_evening);
    if is_evening {
        println!("Good evening!");
    }
}
