advent_of_code::solution!(9);

fn parse_line_to_sequence(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse().expect("Failed to parse sequence"))
        .collect()
}

fn find_next_number(sequence: &[i32]) -> i32 {
    let mut layers = vec![sequence.to_vec()];

    // get down to layers with only zeros
    while !layers.last().unwrap().iter().all(|&x| x == 0) && layers.last().unwrap().len() > 1 {
        let last_layer = layers.last().unwrap();
        let new_layer = last_layer
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();
        layers.push(new_layer);
    }

    // get next number
    for i in (1..layers.len()).rev() {
        let last_num = layers[i - 1].last().unwrap() + layers[i].last().unwrap();
        layers[i - 1].push(last_num);
    }

    *layers[0].last().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum_of_next_numbers = 0;

    for line in lines {
        let sequence = parse_line_to_sequence(line);
        sum_of_next_numbers += find_next_number(&sequence);
    }

    Some(sum_of_next_numbers as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
