// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?

// TODO: Fix the compiler error by updating the function signature.
// 'a is a lifetime parameter — a label for a span of time a borrow is valid.
// Corrected mental model

// Wrong:  'a means x and y live the same length always
// Right:  for this call, both borrows must be valid for 'a,
//         and the return can't outlive 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}



// this is when lifetimes would matter:
// let s1 = String::from("short");
// {
//     let s2 = String::from("longer string");
//     let r = longest(s1.as_str(), s2.as_str());
//     // 'a = roughly "while both s1 and s2 borrows are valid"
//     // r can't outlive this block (s2 dies at end)
// }
// // r can't be used here if it pointed at s2