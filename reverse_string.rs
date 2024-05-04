fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let original = "Hello, world!";
    let reversed = reverse_string(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}
