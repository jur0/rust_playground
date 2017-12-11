pub mod rust_mod;

#[cfg(test)]
mod tests {

    use super::rust_mod::A;

    #[test]
    fn new() {
        let a = A::new();
        assert_eq!(a.A::get_x(), 0);
        assert_eq!(a.A::get_y(), 0);
    }

    #[test]
    fn set() {
        let mut a = A::new();
        a.set(10, 20);
        A::set(&a, 15, 25);
        assert_eq!(a.A::get_x(), 15);
        assert_eq!(a.A::get_y(), 25);
    }
}
