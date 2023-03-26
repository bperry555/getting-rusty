
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


fn my_mode(numbers: &Vec<i32>) -> Option<i32> {

    let mut accumilator = numbers.iter().fold(HashMap::new(),  |mut acc,  value| {
        *acc.entry(value).or_insert(0) +=1;
        acc
    });

    let mode = accumilator
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value,_)| *value);
    return mode
}

fn main() {
   let mut numbers: Vec<i32> = vec![1, 4, 9, 5, 10, 70, 94, 52, 2, 44, 5];

   println!("Mean => {}", my_mean(&numbers));
   println!("Median => {}", my_median(&mut numbers));
   println!("Mode => {:?}", my_mode(&numbers));
   
}
