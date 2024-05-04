fn find_first_occurrence_bruteforce(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &num) in arr.iter().enumerate() {
        if num == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let sorted_array = vec![1, 2, 3, 3, 3, 4, 5, 6, 7, 8];
    let target = 3;

    match find_first_occurrence_bruteforce(&sorted_array, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
