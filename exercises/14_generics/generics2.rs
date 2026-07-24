// This powerful wrapper provides the ability to store a positive integer value.
// TODO: Rewrite it using a generic so that it supports wrapping ANY type.
struct Wrapper<t> {
    value: t,
}

// TODO: Adapt the struct's implementation to be generic over the wrapped value.
// in impl blocks you need to use <t> twice. the 2nd one is for the generic struct named Wrapper<T>
// the generic param is declared on impl
impl<t> Wrapper<t> {
    fn new(value: t) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}