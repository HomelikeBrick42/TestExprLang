fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod sanity_checks {
    #[test]
    fn is_true() {
        assert!(true);
    }
}
