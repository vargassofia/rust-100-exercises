// This is the first excersise about calculator in Rust
fn intro() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to build a calculator in Rust!"
}

// Testing environment
#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to build a calculator in Rust!");
    }
}
// Every binary crate must have a main function
fn main() {
    let message :&str = intro();
    println!("{}", message)

}