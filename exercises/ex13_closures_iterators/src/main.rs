fn example_closure() {
    let add = |a, b| -> i32 { a + b };
    println!("Result: {}", add(2, 3));
}

fn example_iterators() {
    let numbers = vec![1, 2, 3, 4, 5];
    // Explicitly specify the type for the collected iterator
    let doubled: Vec<i32> = numbers.into_iter().map(|n| n * 2).collect();
    println!("Doubled: {:?}", doubled);
}

fn closure_as_parameter<F>(a: i32, b: i32, func: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    func(a, b)
}

fn main() {
    println!("Exercise 13: Closures and Iterators");
    println!("-----------------");
    example_closure();
    example_iterators();
    
    let result = closure_as_parameter(4, 5, |x, y| x * y);
    println!("Closure as parameter result: {}", result);
}
