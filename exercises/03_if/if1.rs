fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a > b {
        return a;
    }

    if b > a {
        return b;
    };

    a
}

fn main() -> Result<(), std::io::Error>{
    let price: i32 = 90;
    let discount = if price > 100 { 20 } else { 5 };
    println!("Discount: {}", discount);
    Ok(())
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}


// What the semicolon does
// The semicolon is the main switch between "keep this value" and "throw it away."

// 5 + 3      // expression → value is 8
// 5 + 3;     // statement → compute 8, discard it → effectively ()
// So ; turns an expression into a statement by discarding its value. That discarded "nothing useful" is typed as () (unit).

// This is why:

// price - 20     // expression → i64
// price - 20;    // statement → ()
// And why the last line of a function matters:

// fn foo() -> i32 {
//     42      // expression → function returns 42
// }
// fn bar() -> i32 {
//     42;     // statement → function tries to return () → compile error
// }