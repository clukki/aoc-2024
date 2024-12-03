use regex::Regex;

pub fn part1(input: String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();
    let mut groups: Vec<(i32, i32)> = vec![];
    for capture in re.captures_iter(&input) {
        let (_, [group]) = capture.extract();
        let mut group_split = group.split(',').map(|e| e.parse::<i32>().unwrap());
        groups.push((group_split.next().unwrap(), group_split.next().unwrap()));
    }

    let mut sum = 0;
    for cap in groups {
        sum += cap.0 * cap.1;
    }

    sum
}

pub fn part2(input: String) -> i32 {
    // added empty captures to circumvent errors about inconsistent group count for each match
    let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)|don\'t\(\)()|do\(\)()").unwrap();
    let mut groups: Vec<(i32, i32)> = vec![];
    let mut enabled = true;
    for capture in re.captures_iter(&input) {
        let (whole, [group]) = capture.extract();
        if whole == "do()" {
            enabled = true;
            continue;
        } else if whole == "don't()" {
            enabled = false;
            continue;
        }

        if !enabled {
            continue;
        }

        let mut group_split = group.split(',').map(|e| e.parse::<i32>().unwrap());
        groups.push((group_split.next().unwrap(), group_split.next().unwrap()));
    }

    let mut sum = 0;
    for cap in groups {
        sum += cap.0 * cap.1;
    }

    sum
}
