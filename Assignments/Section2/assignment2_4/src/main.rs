// Assignment 2.4: Type-Safe Statistics

fn main() {
    let data = vec!["10", "20", "30", "40", "255"];
    let u8_values = data
        .iter()
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let i32_values: Vec<i32> = data.iter().map(|n| n.parse().unwrap()).collect();

    let u16_values: Vec<u16> = u8_values.iter().map(|n| *n as u16).collect();

    let safe_sum = u16_values[0] + u16_values[1] + u16_values[2] + u16_values[3] + u16_values[4];

    let count: f64 = i32_values.len() as f64;
    let f64_values: Vec<f64> = i32_values.iter().map(|n| *n as f64).collect();

    let float_mean =
        (f64_values[0] + f64_values[1] + f64_values[2] + f64_values[3] + f64_values[4]) / count;

    println!("Safe Sum: {}\tFloat Mean: {}", safe_sum, float_mean);
}
