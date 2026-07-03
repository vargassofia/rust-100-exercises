// We create a function that adds the two input numbers 
// and multiplies them by 4
fn compute(a: u32, b: u32) -> u32 {
    let multiplier: u32 = 4;
    a + b * multiplier
}

fn main() {
    let compute: u32 = compute(5, 10);
    println!("The result is: {}", compute)
}