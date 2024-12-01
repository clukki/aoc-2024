use std::collections::{hash_map::Entry, HashMap};

fn parse_to_2_vecs(input: String) -> (Vec<i32>, Vec<i32>) {
    let lines = input.lines();
    let mut arr1: Vec<i32> = vec![];
    let mut arr2: Vec<i32> = vec![];

    for line in lines {
        let mut nums = line.split_whitespace();
        arr1.push(nums.next().unwrap().parse::<i32>().unwrap());
        arr2.push(nums.next().unwrap().parse::<i32>().unwrap());
    }

    (arr1, arr2)
}

pub fn part1(input: String) -> i32 {
    let (mut arr1, mut arr2) = parse_to_2_vecs(input);

    arr1.sort();
    arr2.sort();

    let mut diffs: Vec<i32> = vec![];

    for (num1, num2) in arr1.iter().zip(arr2.iter()) {
        diffs.push((num1 - num2).abs());
    }

    diffs.iter().sum()
}

pub fn part2(input: String) -> i32 {
    let (arr1, arr2) = parse_to_2_vecs(input);

    let mut freq_map: HashMap<i32, i32> = HashMap::new();

    for num in arr2 {
        if let Entry::Vacant(e) = freq_map.entry(num) {
            e.insert(1);
        } else {
            let x = freq_map.get_mut(&num).unwrap();
            *x += 1;
        }
    }

    let mut score = 0;
    for num in arr1 {
        score += num * freq_map.get(&num).unwrap_or(&0);
    }

    score
}
