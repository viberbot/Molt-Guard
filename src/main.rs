mod prompt_guard;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sanity() {
        assert_eq!(1 + 1, 2);
    }
}