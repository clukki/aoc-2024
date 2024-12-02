use std::iter::once;

fn level_safe_check(levels: Vec<i32>) -> bool {
    let levels_iter = levels.iter();

    let mut max = 0;
    let mut inc_result = true;
    for (i, level) in levels_iter.clone().enumerate() {
        if i != 0 && level <= &max {
            inc_result = false;
            break;
        }

        max = *level;
    }

    let mut min = 0;
    let mut dec_result = true;
    for (i, level) in levels_iter.clone().enumerate() {
        if i != 0 && level >= &min {
            dec_result = false;
            break;
        }

        min = *level;
    }

    let sort_result = inc_result || dec_result;

    // adjacent checking
    let mut prev_level = 0;
    let mut adjacent_result = true;
    for (i, level) in levels_iter.clone().enumerate() {
        if i != 0 {
            let abs = (level - prev_level).abs();
            if !((1..=3).contains(&abs)) {
                adjacent_result = false;
                break;
            }
        }

        prev_level = *level
    }

    sort_result && adjacent_result
}

pub fn part1(input: String) -> i32 {
    let reports = input.lines();

    let mut safe_count = 0;
    for report in reports {
        let levels = report
            .split_whitespace()
            .map(|level| level.parse::<i32>().unwrap());

        if level_safe_check(levels.collect()) {
            safe_count += 1;
        }
    }

    safe_count
}

pub fn part2(input: String) -> i32 {
    let reports = input.lines();

    let mut safe_count = 0;
    for report in reports {
        let levels = report
            .split_whitespace()
            .map(|level| level.parse::<i32>().unwrap());

        for (i, _) in levels.clone().enumerate() {
            let mut levels_vec_dampener: Vec<i32> = levels.clone().collect();
            levels_vec_dampener.remove(i);

            if level_safe_check(levels_vec_dampener) {
                safe_count += 1;
                break;
            }
        }
    }

    safe_count
}
