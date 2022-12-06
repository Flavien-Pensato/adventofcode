use crate::read_lines::read_lines;

fn first_part() {
    if let Ok(lines) = read_lines("src/day1/input.txt") {
        let mut max = 0;
        let mut current = 0;

        for line in lines {
            if let Ok(weight) = line {
                if weight == String::from("") {
                    if current > max {
                        max = current
                    }

                    current = 0;
                } else {
                    current += weight.parse().unwrap_or(0)
                }
            }
        }

        println!("How many total Calories is that Elf carrying?\n {}", max);
    }
}

fn second_part() {
    if let Ok(lines) = read_lines("src/day1/input.txt") {
        let mut elve_carying_list = Vec::new();
        let mut current = 0;

        for line in lines {
            if let Ok(weight) = line {
                if weight == String::from("") {
                    elve_carying_list.push(current);
                    current = 0;
                } else {
                    current += weight.parse().unwrap_or(0)
                }
            }
        }

        elve_carying_list.sort();
        elve_carying_list.reverse();

        println!(
            "How many Calories are those Elves carrying in total?\n {:?}",
            &elve_carying_list[..3].iter().sum::<i32>()
        );
    }
}

pub fn day1() {
    first_part();
    second_part();
}
