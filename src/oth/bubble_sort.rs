// bubble_sort.rs
use std::env;

fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}

fn main() {
    let mut nums: Vec<i32> = env::args().skip(1).filter_map(|x| x.parse().ok()).collect();
    nums = bubble_sort(nums);
    println!("Sorted: {:?}", nums);
}
