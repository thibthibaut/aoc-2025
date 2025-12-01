use sscanf::sscanf;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut current = 50;
    let mut counter = 0;
    for line in input.lines() {
        let (dir, num) = sscanf!(line, "{char}{i32}").expect("Error while parsing");
        match dir {
            'L' => current -= num,
            'R' => current += num,
            _ => panic!("parsing error unexpected direction"),
        }
        current %= 100;
        if current == 0 {
            counter += 1
        }
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut current = 50;
    let mut zero_pass = 0;
    for line in input.lines() {
        let (dir, num) = sscanf!(line, "{char}{i32}").expect("Error while parsing");

        zero_pass += match dir {
            'L' => ((100 - current) % 100 + num) / 100, // Force positive
            'R' => (current + num) / 100,
            _ => panic!("parsing error unexpected direction"),
        };

        match dir {
            'L' => current -= num,
            'R' => current += num,
            _ => panic!("parsing error unexpected direction"),
        };
        current = ((current % 100) + 100) % 100;
    }
    Some(zero_pass)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
