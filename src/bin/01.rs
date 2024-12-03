advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(part1), Some(part2)) = (parts.next(), parts.next()) {
            if let (Ok(num1), Ok(num2)) = (part1.parse::<u32>(), part2.parse::<u32>()) {
                list1.push(num1);
                list2.push(num2);
            }
        }
    }    
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        part_one(&advent_of_code::template::read_file("examples", DAY));
        //assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
