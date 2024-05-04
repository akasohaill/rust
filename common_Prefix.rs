fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_string = strings[0];
    let mut prefix = String::new();

    'outer: for (i, ch) in first_string.chars().enumerate() {
        for s in &strings[1..] {
            if let Some(c) = s.chars().nth(i) {
                if c != ch {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        prefix.push(ch);
    }

    prefix
}

fn main() {
    let strings = ["flower", "flow", "flight"];
    println!("Longest common prefix: {}", longest_common_prefix(&strings));

    let strings2 = ["dog", "racecar", "car"];
    println!("Longest common prefix: {}", longest_common_prefix(&strings2));
}
