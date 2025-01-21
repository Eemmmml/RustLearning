// 给定一组整数，使用动态数组来计算该组整数中的平均数、中
// 位数（对数组进行排序后位于中间的值）及众数（出现次数最多的值；哈希映射可以在这里帮上忙）
use std::collections::HashMap;

fn main() {
    let mut nums = vec![1, 2, 2, 2, 3, 4, 5, 6, 6, 6, 2, 4, 6, 6];
    nums.sort();
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

    let mut mid: f64 = 0.0;
    if len % 2 == 0 {
        mid = ((nums[len / 2] as f64) + (nums[len / 2 - 1] as f64)) / (2 as f64);
    } else {
        mid = nums[len / 2] as f64;
    }

    println!("Average is {}", average);
    println!("Mid is {}", mid);
    print!("Max Value: ");
    for i in result {
        print!("{} ", i);
    }
}
