// For a series of integers, use vectors to find the mean, median, 
// and most frequently occurring value.
// 일련의 정수에 대해 벡터를 이용해 평균값, 중간값, 그리고 가장 자주 나타난 값을 구하자.

use std::collections::HashMap;

fn main() {
    // 1. find average
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 11, 12];

    // v -> total
    let sum: i32 = v.iter().sum();
    let len = v.len() as i32;

    let average = sum as f32 / len as f32;

    println!("average: {}", average);

    // 2. find median

    let median = sum / len;
    
    println!("median: {}", median);

    // 3. most frequently occurring value.

    let mut frequency_map = HashMap::new();

    for &num in &v {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    let mut most_frequent_value = 0; // 가장 자주 나타나는 값
    let mut max_frequency = 0; // 가장 자주 나타나는 횟수

    for (&num, &frequency_map) in &frequency_map {
        if frequency_map > max_frequency {
            max_frequency = frequency_map;
            most_frequent_value = num; 
        }
    }

    println!("most_frequent_value: {}", most_frequent_value);

}
