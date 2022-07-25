fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn exclude_test() {
        assert!(false, "this should be excluded")
    }
}
