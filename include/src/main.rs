fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn include_test() {
        assert!(true, "this should be included")
    }
}
