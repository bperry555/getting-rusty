
// Given a list of integers, use a vector and return the median 
// (when sorted, the value in the middle position) and mode (the value that occurs most often; 
// a hash map will be helpful here) of the list. 

use std::collections::HashMap;

fn my_mean(numbers: &Vec<i32>) -> f32 {

    let total: i32 = numbers.iter().sum();
    total as f32 / numbers.len() as f32
}

fn my_median(numbers: &mut Vec<i32>) -> i32 {
    
    numbers.sort();
    let  index = numbers.len() / 2;

    if numbers.len() % 2 != 0 {
        numbers[index]
    } else{
        my_mean(&vec![numbers[index -1], numbers[index]]) as i32
    }
}

fn my_mode(numbers: &Vec<i32>) -> Vec<i32> {

    let mut accumilator = HashMap::new();
    for num in numbers {
        let count = accumilator.entry(num).or_insert(0);
        *count += 1;
    }


    let max_count = accumilator.values().cloned().max().unwrap_or(0);

    accumilator.into_iter()
        .filter(|&(_, v)| v == max_count)
        .map(|(&k, _)| k)
        .collect()
}

fn main() {
   let mut numbers: Vec<i32> = vec![1, 4, 9, 5, 10, 70, 94, 52, 2, 44, 5];

   println!("Mean => {}", my_mean(&numbers));
   println!("Median => {}", my_median(&mut numbers));
   println!("Mode => {:?}", my_mode(&numbers));
   
}
