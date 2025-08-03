use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let mut sum: i32 = 0;
    for num in list {
        sum = sum + *num;
    }
    
    let count = list.len();
    
    if count == 0 {
        return 0.0;
    } else {
        let sum_f64 = sum as f64;
        let count_f64 = count as f64;
        return sum_f64 / count_f64;
    }
}

pub fn median(list: &[i32]) -> i32 {
    if list.len() == 0 {
        return 0;
    }
    
    let mut sorted = Vec::new();
    for num in list {
        sorted.push(*num);
    }
    
    sorted.sort();
    
    let len = sorted.len();
    
    if len % 2 == 1 {
        return sorted[len / 2];
    } 
    else {
        let middle1 = sorted[len / 2 - 1];
        let middle2 = sorted[len / 2];
        return (middle1 + middle2) / 2;
    }
}

pub fn mode(list: &[i32]) -> i32 {
    if list.len() == 0 {
        return 0; 
    }
    
    let mut freq = HashMap::new();
    
    for i in 0..list.len() {
        let num = list[i];
        if freq.contains_key(&num) {
            let current_count = freq.get(&num).unwrap();
            freq.insert(num, current_count + 1);
        } else {
            freq.insert(num, 1);
        }
    }
    
    let mut max_count = 0;
    let mut mode_value = list[0];
    
    for (num, count) in &freq {
        if *count > max_count {
            max_count = *count;
            mode_value = *num;
        }
    }
    
    return mode_value;
}