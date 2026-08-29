fn sort_numbers_inplace(arr: &mut [i32]) {
    let len = arr.len();
    
    if len <= 1 {
        return;
    }

    for i in 0..len - 1 {
        for j in i+1..len {
            if arr[i] > arr[j] {
                arr.swap(i,j);
            }
        }
    }
}

fn sort_numbers(arr: &[i32]) -> Vec<i32> {
    let mut result = arr.to_vec();
    sort_numbers_inplace(&mut result);
    result
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
    println!("Exercise 06: Borrowing");
    println!("------------------");
    
    let mut arr = [5, 3, 8, 4, 2];
    println!("Original array: {:?}", arr);
    println!("Is the array sorted? {}", if check_sorted(&arr) { "Yes" } else { "No" });

    sort_numbers_inplace(&mut arr);
    println!("Sorted array: {:?}", arr);

    let mut new_vec =arr.to_vec();
    new_vec.push(1);

    println!("New vector after adding 1: {:?}", new_vec);

    let new_vec_after_sort = sort_numbers(&new_vec);

    println!("Original array at the end: {:?}", arr);
    println!("Vector at the end: {:?}", new_vec);

    println!("Sorted vector: {:?}", new_vec_after_sort);
    println!("Is the original new vector sorted? {}", if check_sorted(&new_vec) { "Yes" } else { "No" });
    println!("Is the vector sorted? {}", if check_sorted(&new_vec_after_sort) { "Yes" } else { "No" });
    println!("Is the original array sorted? {}", if check_sorted(&arr) { "Yes" } else { "No" });

    loop {
        println!("Press Enter to exit...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        break;
    }
}