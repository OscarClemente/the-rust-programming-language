use std::collections::HashMap;

fn main() {
    let mut v = vec![5, 9, 1, 34, 12, 53, 21, 5, 10, 75, 23, 14, 64, 61, 75, 12, 6, 7, 64, 5];
    println!("Vector values: {:?}", v);

    let mean = calculate_mean(&v);
    println!("The mean is: {}", mean);

    let median = calculate_median(&mut v);
    println!("The median is: {}", median);

    let mode = calculate_mode(&v);
    println!("The mode is: {}", mode);
}

fn calculate_mean(v: &[i32]) -> f32 {
    v.iter().sum::<i32>() as f32 / v.len() as f32
}

fn calculate_median(v: &mut [i32]) -> i32 {
    v.sort();
    let mid = v.len() / 2;
    v[mid]
}

fn calculate_mode(v: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();

    for &num in v {
        let count = occurrences.entry(num).or_insert(0);
        *count += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}