fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None; // k exceeds the length of the array
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort(); // Sort the array in ascending order

    Some(sorted_arr[k - 1]) // Return the kth smallest element (zero-based indexing)
}

fn main() {
    let arr = [7, 10, 4, 3, 20, 15];
    let k = 3;
    match kth_smallest(&arr, k) {
        Some(val) => println!("The {}th smallest element is: {}", k, val),
        None => println!("Invalid input or k is out of range"),
    }
}
