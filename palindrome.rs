fn is_palindrome_direct(input: &str) -> bool {
    let input = input.to_lowercase(); // Convert to lowercase for case-insensitive comparison
    let chars: Vec<char> = input.chars().collect();

    let mut start = 0;
    let mut end = chars.len() - 1;

    while start < end {
        if chars[start] != chars[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    true
}
