// Assignment 3.4: Data Processing Pipeline

fn process_imperative(data: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for item in data {
        if *item >= 0 {
            sum += item.pow(2);
        }
    }
    sum
}

fn process_functional(data: &Vec<i32>) -> i32 {
    data.iter().filter(|x| **x >= 0).map(|x| x.pow(2)).sum()
}

fn main() {
    let raw_data = vec![1, -2, 3, -4, 5, 6, -7, 8];

    let res = process_functional(&raw_data);
    assert_eq!(process_imperative(&raw_data), res);
    println!("Result: {}", res);
}
