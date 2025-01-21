use std::collections::HashMap;

fn main() {
    let nums = vec![1, 2, 2, 2, 3, 4, 5, 6, 6, 6];
    let mut sum = 0;
    let mut statistic = HashMap::new();

    let mut max = 0;
    for i in &nums {
        sum += *i;
        let count = statistic.entry(*i).or_insert(0);
        *count += 1;
        if *count > max {
            max = *count;
        }
    }

    let len = nums.len();
    let average: f64 = (sum as f64) / (len as f64);

    let mut result: Vec<usize> = Vec::new();
    for (&num, &count) in &statistic {
        if count == max {
            result.push(num);
        }
    }

    println!("Average is {}", average);
    print!("Max Value: ");
    for i in result {
        print!("{} ", i);
    }
}
