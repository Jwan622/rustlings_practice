fn call_me(num: u8, phrase: &str) {
    for i in 0..num {
        println!("Ring! {}! Call number {}", phrase,i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(5, "Hello");
}
