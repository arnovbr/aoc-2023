use std::collections::HashMap;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {

    // Parse input
    let mut lines = input.lines();
    let steps = lines.next()?;

    // Skip the empty line
    lines.next();

    let mut map = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue; // Skip any additional empty lines (end)
        }

        let parts: Vec<&str> = line.split(" = ").collect();
        if parts.len() != 2 {
            eprintln!("Invalid line format: {}", line);
            return None;
        }

        let key = parts[0];
        let values_part = parts[1].trim_matches(|p| p == '(' || p == ')');
        let values: Vec<&str> = values_part.split(", ").collect();
        if values.len() != 2 {
            eprintln!("Invalid values format: {}", values_part);
            return None;
        }

        map.insert(key, (values[0], values[1]));
    }

    // iterate starting at AAA ending at ZZZ and counting in between

    let mut current_step = "AAA";
    let mut steps_count = 0;
    let steps_length = steps.len();
    
    if steps_length == 0 {
        eprintln!("No steps provided.");
        return None;
    }
    
    loop {
        if current_step == "ZZZ" { break; }
    
        // Safely get the character for the current step
        let step_char = steps.chars().nth(steps_count % steps_length).expect("Invalid step index");
    
        let (left, right) = map.get(current_step).unwrap();
        current_step = match step_char {
            'L' => left,
            'R' => right,
            _ => panic!("Invalid step: {}", step_char),
        };
    
        steps_count += 1;
    }
    Some(steps_count as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    print!("{}", "sd");
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
