
fn sort_numbers(arr: &[i32] ) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    result.extend_from_slice(arr);

    if result.is_empty()  {
        return result;
    }

    for i in 0..result.len() - 1 {
        for j in i+1..result.len() {
            if result[i] > result[j] {
                result.swap(i,j);
            }
        }
    }

    return result;
}

fn check_sorted(arr: &[i32]) -> bool {
    
    if arr.is_empty()  {
        return true;
    }

    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    return true;
}

fn main() {
    println!("Exercise 2: Functions");
    println!("------------------");
    
    let arr = [5, 3, 8, 4, 2];
    println!("Original array: {:?}", arr);
    println!("Is the array sorted? {}", if check_sorted(&arr) { "Yes" } else { "No" });

    let sorted_arr = sort_numbers(&arr);
    println!("Sorted array: {:?}", sorted_arr);

    let mut sorted_arr = sorted_arr;
    sorted_arr.push(1);

    println!("Sorted array after adding 1: {:?}", sorted_arr);

    sorted_arr = sort_numbers(&sorted_arr);

    println!("Sorted array after sorting again: {:?}", sorted_arr);

    println!("Is the array sorted? {}", if check_sorted(&sorted_arr) { "Yes" } else { "No" });

    loop {
        println!("Press Enter to exit...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        break;
    }
}