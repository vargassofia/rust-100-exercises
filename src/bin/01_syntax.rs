// To Do: fix the function signature below to make the test pass 

// The input parameters should have the same type of the return type. 
fn compute(a: u32, b: u32) -> u32 {
    a + b * 2
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1,2), 5)
    }
}
// Adding an empty main function to satisfy the Rust compiler and rust-analyzer
// since this file is located in a binary directory
fn main() {}