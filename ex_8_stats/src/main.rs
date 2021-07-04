fn main() {
    let mut v = vec![0, 1, 3, 3, 2, 2, 5, 2, 1, 2, 2, 4, 2];

    println!("the average is: {}", mean(&v));
    println!("the median is: {}", median(&mut v));
    println!("the mode is: {}", mode(&v));
}

fn mean(numbers: &[i32]) -> f64 {
    return numbers.iter().sum::<i32>() as f64 / numbers.len() as f64;
}

fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort_unstable();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn mode(numbers: &[i32]) -> i32 {
    use std::collections::HashMap;

    let mut occurrences = HashMap::new();
    for value in numbers.iter() {
        *occurrences.entry(value).or_insert(0) += 1
    }

    *occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}
