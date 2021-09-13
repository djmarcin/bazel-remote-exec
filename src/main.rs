fn say_hello() {
    println!("Hello, world!")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::say_hello();
        assert!(false);
    }
}
