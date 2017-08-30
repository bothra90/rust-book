use std::collections::HashMap;

// This is the public function from this module.
pub fn ls(ints: Vec<i32>) -> (f32, i32, i32) {
    if ints.len() == 0 {
        panic!("Empty vector");
    }
    let mean = mean(&ints);
    let median = median(&ints);
    let mode = mode(&ints);
    (mean, median, mode)
}

fn mean(ints: &Vec<i32>) -> f32 {
    let mut sum = 0; // chances of overflow?
    for v in ints {
        // Following works:
        sum = sum + v;
        // However, following does not work. WHY is that?
        // sum += v;
    }
    println!("Sum is {}", sum);
    sum as f32 / ints.len() as f32
}

fn median(ints: &Vec<i32>) -> i32 {
    let mut ints_copy = ints.clone();
    {
        let int_slice = &mut ints_copy[..];
        int_slice.sort();
    }
    println!("Sorted vector: {:?}", &ints_copy);
    if ints.len() % 2 == 1 {
        ints_copy[ints.len() / 2]
    } else {
        (ints_copy[ints.len() / 2] + ints_copy[(ints.len() / 2) - 1]) / 2
    }
}

fn mode(ints: &Vec<i32>) -> i32 {
    // First, get count of each element.
    let mut counts = HashMap::new();
    for v in ints {
        let count = counts.entry(*v).or_insert(0);
        *count += 1;
    }
    let mut max: (i32, usize) = (0, 0);
    for (k, v) in counts {
        if v > max.1 {
            max = (k, v);
        }
    }
    println!("Count of {} is {}", max.0, max.1);
    max.0
}
